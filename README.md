# shuttle_upload_to_postgres_and_get
This is working postgres database for shuttle with full working upload and fetch from db

# First of all clone this repo and navigate its folder. If there is ```.shuttle``` folder into the project then delete that folder
 * Then authorize the terminal with following command
   ```shuttle login```
 * After that enter following code
     ```shuttle deploy```
 * Then it will ask to replace existing project or create new project, and navigate to the `CREATE NEW PROJECT` and select that and type project name.
   It will create hidden folder ```.shuttle``` into the main project folder.
 * Then project should automatically deployed. If not then enter ```shuttle deploy```
 * After that to upload image enter following command in the terminal
   ```
   curl -X POST -F "file=@/home/rkant/Downloads/Logo.png" https://postgres-image-upload-plye.shuttle.app/
   ```
 * After that enter following url to fetch the image in browser
   ```
   https://postgres-image-upload-plye.shuttle.app//1
   ```
   Note in above code there is double ```//```. This is because there is ```#[get("/{id}")]``` in the get method so ```/``` added up. To not get double ```/``` i need to name something in ```web::scope("/")``` to ```web::scope("/newroute")``` and then can type ```...shuttle.app/newroute/1```
