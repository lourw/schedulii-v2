INSERT INTO users VALUES (1, 'Charlie', 'Brown', 'password', 'charliebrown@gmail.com');
INSERT INTO users VALUES (2, 'Ben', 'Lee', 'password', 'asdf@gmail.com');
INSERT INTO users VALUES (3, 'Mason', 'Chung', 'password', 'mchung@gmail.com');
INSERT INTO users VALUES (4, 'Rebecca', 'Aster', 'password', 'rebast@gmail.com');
INSERT INTO events VALUES (1, 'Ribfest', ('2024-12-02 15:00:00'), ('2024-12-02 17:30:00'))
INSERT INTO events VALUES (2, 'Racecar Derby', ('2024-12-02 15:00:00'), ('2024-12-02 17:30:00'))
INSERT INTO events VALUES (3, 'Marathon', ('2024-12-02 15:00:00'), ('2024-12-02 17:30:00'))
INSERT INTO events VALUES (4, 'Baking Contest', ('2024-12-02 15:00:00'), ('2024-12-02 17:30:00'))
INSERT INTO events VALUES (5, 'Appointment', ('2024-12-02 15:00:00'), ('2024-12-02 17:30:00'))
INSERT INTO event_belongs_to VALUES (1, 1)
INSERT INTO event_belongs_to VALUES (1, 2)
INSERT INTO event_belongs_to VALUES (2, 3)
INSERT INTO event_belongs_to VALUES (3, 4)
INSERT INTO event_belongs_to VALUES (4, 5)
