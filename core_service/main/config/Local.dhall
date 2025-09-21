let Config = ./Config.dhall

let local
    : Config.Type
    = { cookieConfig =
        { useHttps = False
        , sameSite = Config.SameSite.strict
        , cookieDomain = "localhost"
        }
      , corsOrigins = [ "http://localhost:3000", "http://127.0.0.1:3000" ]
      , environment = Config.EnvironmentType.local
      , environmentConfig.frontendHost = "http://localhost:3000"
      , features.foo = False
      , sentryDsn = None Text
      , jwtSecret = "my-super-secret"
      }

in  local
