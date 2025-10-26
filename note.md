## API examples

### Upload output file

```sh
curl -X POST "http://localhost:5325/api/robot/upload" `
     -F "file=@.\robot-data-sample\8-tests-1-ko\output.xml" `
     -F "metadata={`"app_name`":`"MyRobotApp`",`"app_version`":`"1.0.0`"};type=application/json"
```
