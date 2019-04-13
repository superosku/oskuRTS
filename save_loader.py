import struct

class FileTypeException(Exception):
    pass


def bin_array_to_int(array):
    length = len(array)
    return sum([
        ord(data) * 2 ** (8 * (length-i-1))
        for i, data in enumerate(array)
    ])


def bin_array_to_float(array):
    return struct.unpack('<f', array)[0]


class DataHandler:
    def __init__(self, binary_data):
        self.binary_data = binary_data

    def pop_u(self, length):
        if len(self.binary_data) < length:
            raise FileTypeException("File too short")
        data = self.binary_data[0:length]
        self.binary_data = self.binary_data[length:]
        return data
        # return bin_array_to_int(data)

    def pop_f32(self):
        return bin_array_to_float(self.pop_u(4))

    def pop_u32(self):
        return bin_array_to_int(self.pop_u(4))

    def pop_u8(self):
        return bin_array_to_int(self.pop_u(1))

    def pop_padded_data_handler(self):
        size = self.pop_u32()
        if len(self.binary_data) < size:
            raise FileTypeException("Padded binary data too short")
        new_data_handler = DataHandler(self.binary_data[:size])
        self.binary_data = self.binary_data[size:]
        return new_data_handler

    def length(self):
        return len(self.binary_data)

    def empty(self):
        return len(self.binary_data) == 0

    def expect_empty(self):
        if not self.empty():
            raise FileTypeException("DataHandler not empty when expected to be")


class Map:
    def __init__(self, data_handler):
        self.width = data_handler.pop_u32()
        self.height = data_handler.pop_u32()
        size = self.height * self.width
        if data_handler.length() != size * 2:
            raise FileTypeException(
                "Invalid data length for map {} != {}".format(
                    size * 2, data_handler.length()
                )
            )

        self.first_layer = [data_handler.pop_u8() for i in range(size)]
        self.second_layer = [data_handler.pop_u8() for i in range(size)]
        data_handler.expect_empty()

    def first_layer_to_s(self, integer):
        return 'X: .#'[integer]

    def second_layer_to_s(self, integer):
        return ' Xt.'[integer]

    def pretty_print(self):
        for y in range(self.height):
            print(''.join([
                self.first_layer_to_s(self.first_layer[y * self.width + x]) +
                self.second_layer_to_s(self.second_layer[y * self.width + x])
                for x in range(self.width)
            ]))


class Unit:
    def __init__(self, data_handler):
        self.location_x = data_handler.pop_f32()
        self.location_y = data_handler.pop_f32()
        self._id = data_handler.pop_u32()
        self.entity_type = data_handler.pop_u8()
        self.waypoint_index = data_handler.pop_u32()
        self.orientation = data_handler.pop_u32()
        self.team_id = data_handler.pop_u32()
        self.hp = data_handler.pop_u32()
        self.cooldown = data_handler.pop_u32()

        self.path = []

        path_data_handler = data_handler.pop_padded_data_handler()
        while not path_data_handler.empty():
            self.path.append((
                path_data_handler.pop_f32(),
                path_data_handler.pop_f32(),
            ))
        path_data_handler.expect_empty()

        # Enemy point
        enemy_point_exists = data_handler.pop_u8()
        if enemy_point_exists:
            self.enemy_point = (
                data_handler.pop_f32(),
                data_handler.pop_f32(),
            )
        else:
            data_handler.pop_u32(),
            data_handler.pop_u32(),
            self.enemy_point = None

        # Task
        task_data = data_handler.pop_padded_data_handler()
        self.task = task_data.binary_data

        data_handler.expect_empty()

    def pretty_print(self):
        print(
            'Unit:',
            self.location_x,
            self.location_y,
            self._id,
            self.entity_type,
            self.waypoint_index,
            self.orientation,
            self.team_id,
            self.hp,
            self.cooldown,
            self.path,
            self.enemy_point,
            self.task
        )
        pass


class Building:
    def __init__(self, data_handler):
        self.x = data_handler.pop_u32()
        self.y = data_handler.pop_u32()
        data_handler.expect_empty()

    def pretty_print(self):
        print('Building:', self.x, self.y)


class Projectile:
    def __init__(self, data_handler):
        self.location_x = data_handler.pop_f32()
        self.location_y = data_handler.pop_f32()
        self.start_point_x = data_handler.pop_f32()
        self.start_point_y = data_handler.pop_f32()
        self.end_point_x = data_handler.pop_f32()
        self.end_point_y = data_handler.pop_f32()
        self.angle = data_handler.pop_f32()
        data_handler.expect_empty()

    def pretty_print(self):
        print(
            'Projectile: ',
            self.location_x,
            self.location_y,
            self.start_point_x,
            self.start_point_y,
            self.end_point_x,
            self.end_point_y,
            self.angle
        )


class EntityHolder:
    def __init__(self, data_handler):
        self.id_counter = data_handler.pop_u32()

        self.units = []
        self.projectiles = []
        self.buildings = []

        units_data = data_handler.pop_padded_data_handler()
        while not units_data.empty():
            unit_data = units_data.pop_padded_data_handler()
            self.units.append(Unit(unit_data))

        projectiles_data = data_handler.pop_padded_data_handler()
        while not projectiles_data.empty():
            projectile_data = projectiles_data.pop_padded_data_handler()
            self.projectiles.append(Projectile(projectile_data))

        buildings_data = data_handler.pop_padded_data_handler()
        while not buildings_data.empty():
            building_data = buildings_data.pop_padded_data_handler()
            self.buildings.append(Building(building_data))

        data_handler.expect_empty()

    def pretty_print(self):
        for unit in self.units:
            unit.pretty_print()
        for projectile in self.projectiles:
            projectile.pretty_print()
        for building in self.buildings:
            building.pretty_print()


class GameState:
    def __init__(self):
        with open('saved_game.dat', 'rb') as file_object:
            binary_data = file_object.read()
        data_handler = DataHandler(binary_data)
        
        self._map = Map(data_handler.pop_padded_data_handler())
        self.entity_holder = EntityHolder(data_handler.pop_padded_data_handler())

        data_handler.expect_empty()

    def pretty_print(self):
        self._map.pretty_print()
        self.entity_holder.pretty_print()


if __name__ == '__main__':
    game_state = GameState()
    game_state.pretty_print()


