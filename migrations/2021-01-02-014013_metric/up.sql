-- Your SQL goes here
CREATE TABLE metric(
    id INT AUTO_INCREMENT PRIMARY KEY,
    date DATE NOT NULL,
    time TIME,
    weight FLOAT,
    body_fat FLOAT,
    gut_circum FLOAT,
    waist_circum FLOAT,
    chest_circum FLOAT,
    thigh_circum FLOAT
);
