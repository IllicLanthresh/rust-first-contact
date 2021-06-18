macro_rules! enum_str {
    (enum $enum_name:ident {$($variant:ident),*$(,)?}) => {
        enum $enum_name {
            $($variant),*
        }

        impl $enum_name {
            fn to_string(&self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant)),*
                }
            }
        }
    };
    (
        enum $enum_name:ident {$($variant:ident),*$(,)?},
        $(enum $enum_names:ident {$($variants:ident),*$(,)?}),+$(,)?
    ) => {
        enum_str!{enum $enum_name {$($variant),*}}
        enum_str!{$(enum $enum_names {$($variants),*}),+}
    }
}

// macro_rules! type_from_model {
//     (
//         model_tag: $model_tag:ident
//
//         fields:
//           $($field_name:ident:
//             type: text
//             $(max_length: $max_length:literal)?
//             $(nullable: Yes)?
//             $(description: $description:literal)?
//             $(options:
//               $(- $option:literal)+)?)+
//     ) => {
//         struct $model_tag {
//             $($field_name: String),+
//         }
//     }
// }
// type_from_model! {
// model_tag: vsat_antenna
//
// fields:
//   name:
//     type: text
//     max_length: 30
//
//   serial_number:
//     type: text
//     nullable: Yes
//
//   position:
//     type: text
//     description: "Physical location of the antenna in the vessel"
//     options:
//       - port
//       - starboard
//       - fore
//       - aft
//       - unknown
// }

enum_str! {
    enum ContractStatusOptions {
        Expired,
        Active,
        Inactive,
        Connected,
        Keepalive,
        Suspension
    },
    enum AdminStateOptions {
        Active,
        KeepAlive,
        Spare,
        Storage
    },
}

struct Vessel {
    name: String,
    owner: Option<String>,
    plexus_number: Option<i32>,
    contract_status: Option<ContractStatusOptions>,
}

struct VsatModem {
    name: String,
    serial_number: Option<String>,
    modem_id: String,
    admin_state: AdminStateOptions,
}

fn main() {



//     rust_model_parser::my_macro!{
// model_tag: vsat_antenna
//
// description: "A VSAT antenna provides physical connectivity between VSAT modems and satellite networks."
//
// fields:
//   name:
//     type: text
//     max_length: 30
//
//   serial_number:
//     type: text
//     nullable: Yes
//
//   position:
//     type: text
//     description: "Physical location of the antenna in the vessel"
//     options:
//       - port
//       - starboard
//       - fore
//       - aft
//       - unknown
//     }
//     rust_model_parser::my_macro_from_file!("src/model_definitions/vsat_antenna.model.yml");
}
