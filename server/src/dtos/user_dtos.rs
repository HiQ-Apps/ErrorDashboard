use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::auth_dtos::RefreshTokenDTO;


#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreateDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortUserDTO {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginServiceDTO {
    pub user: ShortUserDTO,
    pub access_token: String,
    pub refresh_token: RefreshTokenDTO
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponseDTO {
    pub user: ShortUserDTO,
    pub access_token: String,
}

impl From<UserLoginServiceDTO> for UserLoginResponseDTO {
    fn from(service_dto: UserLoginServiceDTO) -> Self {
        UserLoginResponseDTO {
            user: service_dto.user,
            access_token: service_dto.access_token,
        }
    }
}

