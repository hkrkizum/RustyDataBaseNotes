-- ページ階層: 自己参照外部キー
ALTER TABLE pages ADD COLUMN parent_id TEXT REFERENCES pages(id) ON DELETE SET NULL;

-- 将来の手動並べ替え用（本スコープでは使用せず DEFAULT 0 のまま）
ALTER TABLE pages ADD COLUMN sort_order INTEGER NOT NULL DEFAULT 0;

-- 子ページ検索の高速化
CREATE INDEX idx_pages_parent_id ON pages(parent_id);
