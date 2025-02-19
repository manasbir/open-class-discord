CREATE TABLE buildings (
    building_code VARCHAR(10) PRIMARY KEY,
    primary_name VARCHAR(255) NOT NULL
);

CREATE TABLE floors (
    floor_id PRIMARY KEY,
    building_code VARCHAR(10) REFERENCES buildings(building_code),
    floor_number INTEGER NOT NULL,
    UNIQUE (building_code, floor_number)
);

CREATE TABLE rooms (
    room_id PRIMARY KEY,
    building_code VARCHAR(10) REFERENCES buildings(building_code),
    floor_id VARCHAR(10) REFERENCES floors(floor_id),
    room_number INTEGER NOT NULL,
    UNIQUE (building_code, room_number)
);

CREATE TABLE time_slots (
    slot_id PRIMARY KEY,
    room_id INTEGER REFERENCES rooms(room_id),
    day VARCHAR(9) NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    UNIQUE (room_id, day, start_time)
);