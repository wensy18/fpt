mod microservice1{
    let internal_data1 = "ms1 secret".to_string();
    let internal_data2 = "ms1 secret".to_string();
    let internal_data3 = "ms1 secret".to_string();
    let interact_data1 = function(internal_data1,internal_data2,internal_data3);

    fn api_call(args:type)->&String {&interact_data1}
}


mod microservice3{
    let internal_data1 = "ms3 secret".to_string();
    let interact_data1 = "ms3 runtime info".to_string();
    let external_data1 = microservice1::api_call();
    let interact_data2 = function(internal_data1,external_data1);

    fn api_call(args:type)->&String {&interact_data2}
    fn api_call2(args:type)->&String {&external_data1}
}

mod microservice4{
    let internal_data1 = "ms4 secret".to_string();
    let external_data1 = microservice3::api_call2();
    
    ...//processing with external_data1

    fn api_call(args:type)->&String {&interact_data2}
}