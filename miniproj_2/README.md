
# mini_project_2
This is a simple AWS Lambda function that processes data.

## Author
Yucheng Yang yy341

## Function
This imple AWS Lambda function times the input number with π. The value of π is approximately 3.14159. The main function in file main.rs can handle the API Gateway. The project is deloyed in AWS console. In IAM service page, access key ID and secret access key are created and saved in ~/.aws/credentials. And a new role with the policy of AWSLambda_FullAccess is created. This function can be found in my AWS account.

## Screenshots
![Local Image](/fig/1.jpg)

This is the main structure of this Lambda function, deployed in AWS and integrated with API gateway.

![Local Image](/fig/2.jpg)

This is the API Gateway test of data processing with this function. Input number is 4.

![Local Image](/fig/3.jpg)
This is the response of the function, input 4 is multiplied with π, which returns the result of 12.566...
