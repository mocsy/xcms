
## Check db stuff
```bash
psql postgres -d ecs
```
### Check Project uids
```pgsql
select uuid, title from projects;
select uuid, title from projects where title like '%Milestone%';
```
### Export todos
```pgsql
\copy (select * from todos where completed='t' and project_id = '9e9d8983-15e3-4c99-8175-0c7ed4a61823') to '/tmp/project_tasks_completed.csv' with delimiter ','

\copy (select * from todos where completed='f' and project_id = '9e9d8983-15e3-4c99-8175-0c7ed4a61823') to '/tmp/project_tasks_remaining.csv' with delimiter ','
```
### Copy exports to localhost
```bash
scp root@ecs.dev.reedwolf.com:/tmp/project_tasks_* ./
```
> root@ecs.dev.reedwolf.com's password: 
> project_tasks_completed.csv                    100% 4414   174.1KB/s   00:00
> project_tasks_remaining.csv                    100% 2696   106.4KB/s   00:00

### Drop todos of a Project
```pgsql
delete from todos where project_id='9e9d8983-15e3-4c99-8175-0c7ed4a61823';
```
## Import csv
The csv must have 4 columns:
> name,phone,email,description

```bash
./target/debug/project_loader --project 9e9d8983-15e3-4c99-8175-0c7ed4a61823 --list 0608projects.csv
```

