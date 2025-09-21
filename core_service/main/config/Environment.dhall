let EnvironmentType = < local | development | staging | production >

let EnvironmentConfig =
      { -- Hostname of frontend, including scheme (http or https)
        frontendHost : Text
      }

in  { EnvironmentType, EnvironmentConfig }
