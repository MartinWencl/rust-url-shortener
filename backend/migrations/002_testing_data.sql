DO $$
DECLARE
  id UUID;
  red_id UUID;
BEGIN
  INSERT INTO Users ( name, private_key ) 
  VALUES ( 'Test', '1234' );

  INSERT INTO Users ( name, private_key ) 
  VALUES ( 'Admin', '1234' )
  RETURNING user_id INTO id;

  INSERT INTO Redirects ( url_from, url_to ) 
  VALUES ( 'TestUrlFrom', 'TestUrlTo' )
  RETURNING redirect_id INTO red_id;

  INSERT INTO Redirects_Users ( redirect_id, user_id ) 
  VALUES ( RED_ID, ID );
END $$;
