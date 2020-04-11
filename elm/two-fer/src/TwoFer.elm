module TwoFer exposing (twoFer)


twoFer : Maybe String -> String
twoFer name =

    case name of
        Nothing -> "you"
        String n _ -> n

