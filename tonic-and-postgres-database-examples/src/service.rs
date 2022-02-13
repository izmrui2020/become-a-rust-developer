//
use anyhow::Result;
use chrono::*;
use http::request;
use serde::__private::de::ContentDeserializer;
use uuid::Uuid;
use super::db_connection::estabilish_connection;
use tonic::{Request, Response, Status};

use crate::user::{
    crud_server::Crud
};
use crate::user::*;

#[derive(Debug, Default)]
pub struct User {}

#[tonic::async_trait]
impl Crud for User {
    async fn get_user(&self, request: Request<UserRequest>) -> Result<Response<UserReply>, Status> {
        println!("Got a request: {:#?}", &request);

        let UserRequest { id } = &request.into_inner();

        let mut conn = estabilish_connection();

        let rows = &conn
            .query("SELECT * FROM users WHERE id = $1", &[&id])
            .unwrap();
        
        let row = rows.get(0).unwrap();
        println!("{:#?}", &row);
        
        let date_of_birth: NaiveDate = row.get(3);

        let reply = UserReply {
            id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
            date_of_birth: date_of_birth.to_string(),
        };

        Ok(Response::new(reply))
    }

    fn list_users< 'life0, 'async_trait>(& 'life0 self,request:tonic::Request<crate::user::Empty> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<crate::user::Users> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }


    fn create_user< 'life0, 'async_trait>(& 'life0 self,request:tonic::Request<crate::user::CreateUserRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<crate::user::CreateUserReply> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }


    fn update_user< 'life0, 'async_trait>(& 'life0 self,request:tonic::Request<crate::user::UpdateUserRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<crate::user::UpdateUserReqly> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }


    fn delete_user< 'life0, 'async_trait>(& 'life0 self,request:tonic::Request<crate::user::UserRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<crate::user::DeleteUserReply> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }


    fn delete_users< 'life0, 'async_trait>(& 'life0 self,request:tonic::Request<crate::user::Empty> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<crate::user::DeleteUserReply> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }

}
