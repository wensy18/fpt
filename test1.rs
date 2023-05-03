mod microservice0{
    let internal_data1 = "ms0 secret".to_string();                                            //ok
    let internal_data2 = "ms0 secret".to_string();                                            //ok       

    fn api_call(args:type) -> Bool {return Ture if ... else False}                            //ok
}

mod microservice0{
    let internal_data1 = "ms0 no-secret".to_string();                                         //ok
    let internal_data2 = "ms0 no-secret".to_string();                                         //ok
    let interact_data1 = function(internal_data1,internal_data2);                             //ok

    fn api_call(args:type) -> &String {&interact_data1}                                       //ok
}

mod microservice1{
    let internal_data1 = "ms1 secret".to_string();                                            //ok
    let internal_data2 = "ms1 secret".to_string();                                            //ok
    let internal_data3 = "ms1 secret".to_string();                                            //ok
    let interact_data1 = function(internal_data1,internal_data2,internal_data3);              //ok

    fn api_call(args:type)->&String {&interact_data1}                                         //To check
}
