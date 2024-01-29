CREATE TABLE character.character (
  id INT NOT NULL AUTO_INCREMENT,
-- The name of the character
  name VARCHAR(255) NOT NULL,
-- A string in the format of #RRGGBBAA
  color CHAR(9) NOT NULL,
-- An url to a picture
  portrait VARCHAR(255),
-- The firebase id of the user who owns the character
  owner VARCHAR(255) NOT NULL,
  PRIMARY KEY (id))
DEFAULT CHARACTER SET = utf8;
