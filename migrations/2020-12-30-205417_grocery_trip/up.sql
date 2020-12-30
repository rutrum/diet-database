CREATE TABLE grocery_trip (
    id INT AUTO_INCREMENT PRIMARY KEY,
    date DATE NOT NULL,
    time TIME,
    store_id INT NOT NULL,
    CONSTRAINT store_fk
    FOREIGN KEY (store_id)
        REFERENCES store(id)
)
