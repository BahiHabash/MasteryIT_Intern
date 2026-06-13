--! delete_project_by_key
DELETE FROM projects
WHERE key = :key;

--! upsert_user
INSERT INTO app_users (email, display_name)
VALUES (:email, :display_name)
ON CONFLICT (email) DO UPDATE
SET display_name = EXCLUDED.display_name
RETURNING id, email, display_name;

--! upsert_project
INSERT INTO projects (owner_id, key, name)
VALUES (:owner_id, :key, :name)
ON CONFLICT (key) DO UPDATE
SET name = EXCLUDED.name
RETURNING id, key, name;

--! create_task
INSERT INTO tasks (project_id, assignee_id, title, priority)
VALUES (:project_id, :assignee_id, :title, :priority)
RETURNING id, title, status, priority;

--! complete_task
UPDATE tasks
SET status = 'done',
    completed_at = now()
WHERE id = :task_id
RETURNING id, title, status, priority;

--! task_board
SELECT
    tasks.id,
    tasks.title,
    tasks.status,
    tasks.priority,
    COALESCE(app_users.display_name, 'unassigned') AS assignee_name
FROM tasks
LEFT JOIN app_users ON app_users.id = tasks.assignee_id
WHERE tasks.project_id = :project_id
ORDER BY tasks.status, tasks.priority, tasks.id;

--! project_summary
SELECT
    projects.key,
    projects.name,
    COUNT(tasks.id)::BIGINT AS total_tasks,
    COUNT(tasks.id) FILTER (WHERE tasks.status <> 'done')::BIGINT AS open_tasks,
    COUNT(tasks.id) FILTER (WHERE tasks.status = 'done')::BIGINT AS done_tasks
FROM projects
LEFT JOIN tasks ON tasks.project_id = projects.id
WHERE projects.id = :project_id
GROUP BY projects.id, projects.key, projects.name;
