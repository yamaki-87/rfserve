drop table if exists downloadlinks;

create table downloadlinks (
    id uuid PRIMARY KEY,
    yt_history_id integer not null,
    url TEXT NOT NULL,
    -- 署名付きURL
    object_path TEXT NOT NULL,
    -- どのファイル/リソースへのリンクか
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    -- 有効期限
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);