# gs-cli
A CLI for CRUD operation on a Google Sheet

### All available methods

#### 1. GET
- ```gs get```
- ```
  gs get
   --f k=v
  ```

#### 2. POST
- ```
  gs post
   --d k=v
   --d k=v k1=v1 k2=v2
  ```
  
#### 3. PATCH
- ```
  gs patch
  --i k=v --d k=v k1=v1
  ```
- ```
  gs patch
  --i k=v --d k=v k1=v1
  --i k1=v1 --d k=v
  ```

#### 4. DELETE
- ```
  gs delete
  --i k=v
  --i k1=v1
  --i k2=v2
  ```

### All available modifiers  
```
--i / --id
--d / --data
--f / --filter
```