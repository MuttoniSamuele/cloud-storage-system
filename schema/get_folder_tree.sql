CREATE OR REPLACE FUNCTION get_folder_tree(p_folder_id INT, p_owner_id INT)
RETURNS TABLE(folder_id INT, parent_id INT) AS $$
BEGIN
    RETURN QUERY WITH RECURSIVE folder_tree AS (
        SELECT id, fk_parent
        FROM folders
        WHERE id = p_folder_id AND fk_owner = p_owner_id
        UNION
        SELECT f.id, f.fk_parent
        FROM folders f
        JOIN folder_tree ft ON f.fk_parent = ft.id
    )
    SELECT folder_tree.id, folder_tree.fk_parent FROM folder_tree;
END; $$ 
LANGUAGE plpgsql;
