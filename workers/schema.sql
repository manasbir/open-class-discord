CREATE TABLE buildings (
    building_code VARCHAR(10) PRIMARY KEY,
    primary_name VARCHAR(255) NOT NULL
);

CREATE TABLE floors (
    floor_id PRIMARY KEY,
    building_code VARCHAR(10) REFERENCES buildings(building_code),
    floor_number INTEGER NOT NULL,
    display_name VARCHAR(50) NOT NULL, -- "4th Floor", "Ground Floor", etc.
    UNIQUE (building_code, floor_number)
);

CREATE TABLE rooms (
    room_id PRIMARY KEY,
    building_code VARCHAR(10) REFERENCES buildings(building_code),
    floor_id INTEGER REFERENCES floors(id),
    room_number VARCHAR(20) NOT NULL,
    UNIQUE (building_code, room_number)
);

CREATE TABLE time_slots (
    slot_id PRIMARY KEY,
    room_id INTEGER REFERENCES rooms(id),
    weekday VARCHAR(9) NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    UNIQUE (room_id, weekday, start_time)
);