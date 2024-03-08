CREATE TYPE gender AS ENUM ('female', 'male');
CREATE TYPE stroke AS ENUM ('butterfly', 'back', 'breast', 'freestyle');

CREATE OR REPLACE FUNCTION participant_short_id()
RETURNS integer AS
$$
	DECLARE
		short_id_guess INT;
	BEGIN
		LOOP
			short_id_guess := FLOOR(RANDOM() * 8999) + 1000;
			IF NOT EXISTS (SELECT * FROM participants p WHERE p.short_id = short_id_guess) THEN
				RETURN short_id_guess;
			END IF;
		END LOOP;
	END;
$$ LANGUAGE plpgsql;

CREATE TABLE groups (
	id			UUID			PRIMARY KEY NOT NULL		DEFAULT gen_random_uuid(),
	name		TEXT			NOT NULL
);

CREATE TABLE participants (
	id 			UUID			PRIMARY KEY NOT NULL		DEFAULT gen_random_uuid(),
	short_id	INT				NOT NULL					DEFAULT participant_short_id() UNIQUE,
	first_name	TEXT			NOT NULL,
	last_name	TEXT			NOT NULL,
	gender		gender			NOT NULL,
	birthday	DATE			NOT NULL,
	group_id	UUID			NOT NULL					REFERENCES groups(id)
);

CREATE TABLE competitions (
	id			UUID			PRIMARY KEY NOT NULL		DEFAULT gen_random_uuid(),
	gender		gender			NOT NULL,
	stroke		stroke			NOT NULL,
	distance	INT				NOT NULL 					CHECK((distance % 25) = 0 AND distance > 0),
	target_time INT				NOT NULL					CHECK (target_time > 0),
	CONSTRAINT no_same_competitions UNIQUE (gender, stroke, distance)
);

CREATE TABLE registrations (
	id					UUID			PRIMARY KEY NOT NULL		DEFAULT gen_random_uuid(),
	participant_id		UUID			NOT NULL					REFERENCES participants(id),
	competition_id		UUID			NOT NULL					REFERENCES competitions(id),
	CONSTRAINT one_registration_per_participant UNIQUE (participant_id, competition_id)
);

CREATE TABLE registration_results (
	registration_id		UUID			PRIMARY KEY NOT NULL		REFERENCES registrations(id),
	disqualified		BOOL			NOT NULL,
	time_millis			INT				NOT NULL					CHECK (time_millis > 0)
);
