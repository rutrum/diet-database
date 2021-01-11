-- Your SQL goes here
CREATE TABLE grocery_item(
    id INT AUTO_INCREMENT PRIMARY KEY,
    trip_id INT NOT NULL,
    name VARCHAR(100) NOT NULL,
    amount FLOAT,
    measure VARCHAR(100),
    FOREIGN KEY (trip_id)
        REFERENCES grocery_trip(id)
);
