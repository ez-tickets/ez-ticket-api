use axum::extract::Multipart;
use error_stack::{Report, ResultExt};
use kernel::entities::product::{ProductDesc, ProductName, ProductPrice};
use kernel::io::commands::ProductCommand;

use crate::errors::ServerError;

#[derive(Debug)]
#[cfg_attr(feature = "apidoc", derive(utoipa::ToSchema))]
pub struct RegisterProduct {
    name: String,
    desc: String,
    price: i64,
    #[cfg_attr(feature = "apidoc", schema(value_type = String, format = Binary, content_media_type = "application/octet-stream"))]
    image: Vec<u8>
}

impl RegisterProduct {
    // noinspection DuplicatedCode
    pub async fn from_multipart(mut multipart: Multipart) -> Result<Self, Report<ServerError>> {
        let mut name: Option<String> = None;
        let mut desc: Option<String> = None;
        let mut price: Option<i64> = None;
        let mut image: Option<Vec<u8>> = None;

        while let Some(field) = multipart.next_field().await
            .change_context_lazy(|| ServerError::InvalidFormat)?
        {
            let key = field.name()
                .ok_or(ServerError::InvalidFormat)?;

            match key {
                "name" => name = Some(field.text().await
                    .change_context_lazy(|| ServerError::InvalidFormat)?),
                "desc" => desc = Some(field.text().await
                    .change_context_lazy(|| ServerError::InvalidFormat)?),
                "price" => price = Some(field.text().await
                    .change_context_lazy(|| ServerError::InvalidFormat)?
                    .parse::<i64>()
                    .change_context_lazy(|| ServerError::InvalidFormat)?),
                "image" => image = Some(field.bytes().await
                    .change_context_lazy(|| ServerError::InvalidFormat)?.to_vec()),
                _ => {
                    tracing::warn!("unknown field: {}", key);
                    return Err(ServerError::InvalidFormat.into());
                }
            }
        }

        Ok(Self {
            name: name.ok_or(ServerError::InvalidFormat)?,
            desc: desc.ok_or(ServerError::InvalidFormat)?,
            price: price.ok_or(ServerError::InvalidFormat)?,
            image: image.ok_or(ServerError::InvalidFormat)?,
        })
    }
}

impl TryFrom<RegisterProduct> for ProductCommand {
    type Error = Report<ServerError>;

    fn try_from(value: RegisterProduct) -> Result<Self, Self::Error> {
        Ok(ProductCommand::Register {
            name: ProductName::new(value.name),
            desc: ProductDesc::new(value.desc),
            price: ProductPrice::new(value.price).change_context_lazy(|| ServerError::Validation)?,
            image: value.image,
        })
    }
}


#[derive(Debug)]
#[cfg_attr(feature = "apidoc", derive(utoipa::ToSchema))]
pub struct PatchProduct {
    #[cfg_attr(feature = "apidoc", schema(value_type = String))]
    pub name: Option<ProductName>,
    #[cfg_attr(feature = "apidoc", schema(value_type = String))]
    pub desc: Option<ProductDesc>,
    #[cfg_attr(feature = "apidoc", schema(value_type = i64))]
    pub price: Option<ProductPrice>,
    #[cfg_attr(feature = "apidoc", schema(value_type = String, format = Binary, content_media_type = "application/octet-stream"))]
    pub image: Option<Vec<u8>>
}

impl PatchProduct {
    // noinspection DuplicatedCode
    pub async fn from_multipart(mut multipart: Multipart) -> Result<Self, Report<ServerError>> {
        let mut name: Option<String> = None;
        let mut desc: Option<String> = None;
        let mut price: Option<i64> = None;
        let mut image: Option<Vec<u8>> = None;

        while let Some(field) = multipart.next_field().await
            .change_context_lazy(|| ServerError::InvalidFormat)?
        {
            let key = field.name()
                .ok_or(ServerError::InvalidFormat)?;

            match key {
                "name" => name = Some(field.text().await
                    .change_context_lazy(|| ServerError::InvalidFormat)?),
                "desc" => desc = Some(field.text().await
                    .change_context_lazy(|| ServerError::InvalidFormat)?),
                "price" => price = Some(field.text().await
                    .change_context_lazy(|| ServerError::InvalidFormat)?
                    .parse::<i64>()
                    .change_context_lazy(|| ServerError::InvalidFormat)?),
                "image" => image = Some(field.bytes().await
                    .change_context_lazy(|| ServerError::InvalidFormat)?.to_vec()),
                _ => {
                    tracing::warn!("unknown field: {}", key);
                    return Err(ServerError::InvalidFormat.into());
                }
            }
        }

        Ok(Self {
            name: name.map(ProductName::new),
            desc: desc.map(ProductDesc::new),
            price: price.map(ProductPrice::new)
                .transpose()
                .change_context_lazy(|| ServerError::Validation)?,
            image,
        })
    }
}