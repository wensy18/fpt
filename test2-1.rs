mod microservice1{
    let internal_data1 = "ms1 secret".to_string();
    let internal_data2 = "ms1 secret".to_string();
    let internal_data3 = "ms1 secret".to_string();
    let interact_data1 = function(internal_data1,internal_data2,internal_data3);

    fn api_call(args:type)->&String {&interact_data1}

    op_on(interact_data1)
}


mod microservice3{
    let internal_data1 = "ms3 secret".to_string();
    let interact_data1 = "ms3 runtime info".to_string();
    
    let external_data1 = microservice1::api_call();
    ...//processing with external_data1
    ...//in microservice1, data is updated
}