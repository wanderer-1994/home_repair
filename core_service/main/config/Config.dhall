let Cookie = ./Cookie.dhall

let Environment = ./Environment.dhall

let Features = ./Features.dhall

let SameSite = Cookie.SameSite

let CookieConfig = Cookie.CookieConfig

let EnvironmentType = Environment.EnvironmentType

let EnvironmentConfig = Environment.EnvironmentConfig

let FeaturesType = Features.FeaturesType

let ConfigType =
      { cookieConfig : CookieConfig
      , corsOrigins : List Text
      , environment : EnvironmentType
      , environmentConfig : EnvironmentConfig
      , features : FeaturesType
      , sentryDsn : Optional Text
      , -- Secret for signing, verifying JWT token
        jwtSecret : Text
      }

in  { Type = ConfigType
    , CookieConfig
    , EnvironmentType
    , EnvironmentConfig
    , FeaturesType
    , SameSite
    }
