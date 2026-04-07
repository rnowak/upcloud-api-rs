use super::*;
// Source: https://developers.upcloud.com/1.3/7-plans/#list-available-plans
const PLAN_LIST_JSON: &str = r#"
    {
        "plans": {
            "plan": [
                {
                    "core_number": "1",
                    "memory_amount": "2048",
                    "name": "1xCPU-2GB",
                    "public_traffic_out": "2048",
                    "storage_size": "50",
                    "storage_tier": "maxiops"
                },
                {
                    "core_number": "2",
                    "memory_amount": "4096",
                    "name": "2xCPU-4GB",
                    "public_traffic_out": "4096",
                    "storage_size": "80",
                    "storage_tier": "maxiops"
                },
                {
                    "core_number": "32",
                    "memory_amount": "393216",
                    "name": "GPU-32xCPU-384GB-2xL40S",
                    "public_traffic_out": "44",
                    "gpu_amount": 2,
                    "storage_size": "0",
                    "storage_tier": null,
                    "gpu_model": "NVIDIA L40S"
                }
            ]
        }
    }
"#;

#[test]
fn deserialize_plan_list() {
    let resp: PlanListResponse = serde_json::from_str(PLAN_LIST_JSON).unwrap();
    let plans = resp.plans.plan;
    assert_eq!(plans.len(), 3);
    assert_eq!(plans[0].name, "1xCPU-2GB");
    assert_eq!(plans[0].core_number, 1);
    assert_eq!(plans[0].memory_amount, 2048);
    assert_eq!(plans[0].storage_size, 50);
    assert_eq!(plans[0].storage_tier.as_deref(), Some("maxiops"));
    assert_eq!(plans[0].public_traffic_out, 2048);
    assert!(plans[0].gpu_amount.is_none());
    assert!(plans[0].gpu_model.is_none());

    assert_eq!(plans[1].name, "2xCPU-4GB");
    assert_eq!(plans[1].core_number, 2);
    assert_eq!(plans[1].memory_amount, 4096);
    assert_eq!(plans[1].storage_size, 80);
    assert_eq!(plans[1].storage_tier.as_deref(), Some("maxiops"));
    assert_eq!(plans[1].public_traffic_out, 4096);
    assert!(plans[1].gpu_amount.is_none());
    assert!(plans[1].gpu_model.is_none());
}

#[test]
fn deserialize_gpu_plan_null_storage_tier() {
    let resp: PlanListResponse = serde_json::from_str(PLAN_LIST_JSON).unwrap();
    let gpu_plan = &resp.plans.plan[2];
    assert_eq!(gpu_plan.name, "GPU-32xCPU-384GB-2xL40S");
    assert_eq!(gpu_plan.core_number, 32);
    assert_eq!(gpu_plan.memory_amount, 393216);
    assert_eq!(gpu_plan.storage_size, 0);
    assert_eq!(gpu_plan.public_traffic_out, 44);
    assert!(gpu_plan.storage_tier.is_none());
    assert_eq!(gpu_plan.gpu_amount, Some(2));
    assert_eq!(gpu_plan.gpu_model.as_deref(), Some("NVIDIA L40S"));
}
// Source: https://developers.upcloud.com/1.3/8-servers/#list-server-configurations
const SERVER_SIZE_LIST_JSON: &str = r#"
    {
        "server_sizes": {
            "server_size": [
                {
                    "core_number": "1",
                    "memory_amount": "1024"
                },
                {
                    "core_number": "2",
                    "memory_amount": "2048"
                }
            ]
        }
    }
"#;

#[test]
fn deserialize_server_size_list() {
    let resp: ServerConfigurationListResponse =
        serde_json::from_str(SERVER_SIZE_LIST_JSON).unwrap();
    let sizes = resp.server_sizes.server_size;
    assert_eq!(sizes.len(), 2);
    assert_eq!(sizes[0].core_number, 1);
    assert_eq!(sizes[0].memory_amount, 1024);
    assert_eq!(sizes[1].core_number, 2);
    assert_eq!(sizes[1].memory_amount, 2048);
}
