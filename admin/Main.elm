module Main exposing (..)

import Element exposing (..)
import Element.Attributes exposing (..)
import Html
import Login
import Return exposing (..)
import Stylesheet exposing (..)


type Msg
    = MsgForLogin Login.Msg


type alias Model =
    { login : Login.Model }


init : Return Msg Model
init =
    return { login = Login.init } Cmd.none


update : Msg -> Model -> Return Msg Model
update msg model =
    case msg of
        MsgForLogin msg ->
            Login.update msg model.login
                |> Return.map (\login -> { model | login = login })
                |> Return.mapCmd MsgForLogin


main : Program Never Model Msg
main =
    Html.program
        { init = init
        , update = update
        , view = view
        , subscriptions = \_ -> Sub.none
        }


view : Model -> Html.Html Msg
view model =
    Element.layout stylesheet <|
        column General
            [ center, width (percent 100) ]
            [ navigation
            , column None
                [ width (percent 100), maxWidth (px 1200), paddingXY 40 40 ]
                [ Element.map MsgForLogin (Login.view model.login)
                ]
            ]


navigation : Element Styles variation msg
navigation =
    row Navigation
        [ spread, paddingXY 80 20, width (percent 100) ]
        [ el Logo [] (text "Fake News Detector Admin")
        ]
