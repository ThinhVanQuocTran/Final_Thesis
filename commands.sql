CREATE TABLE department (did INT PRIMARY KEY, name VARCHAR(50), year_started INT, year_ended INT);
CREATE TABLE people (pid INT PRIMARY KEY, first_name VARCHAR(50), last_name VARCHAR(50), did INT NOT NULL);
