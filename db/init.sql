DROP TABLE IF EXISTS cpu CASCADE;
DROP TABLE IF EXISTS ram CASCADE;
Create TABLE IF NOT EXISTS cpu (
    date TIMESTAMP NOT NULL,
    core0_u INT,
    core1_u INT,
    core2_u INT,
    core3_u INT,
    core0_t INT,
    core1_t INT,
    core2_t INT,
    core3_t INT
);
Create TABLE IF NOT EXISTS ram (
    date TIMESTAMP NOT NULL,
    used INT,
    total INT
);