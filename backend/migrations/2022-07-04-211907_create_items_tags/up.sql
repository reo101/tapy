CREATE TABLE items_tags (
    item_id INTEGER NOT NULL,
    tag_id  INTEGER NOT NULL,
    CONSTRAINT itema_tags_pk
        PRIMARY KEY (item_id, tag_id),
    CONSTRAINT items_tags_fk_items
        FOREIGN KEY (item_id)
            REFERENCES items(id)
            ON DELETE CASCADE,
    CONSTRAINT items_tags_fk_tags
        FOREIGN KEY (tag_id)
            REFERENCES tags(id)
)
