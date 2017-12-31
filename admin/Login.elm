module Login exposing (..)

import Element exposing (..)
import Element.Attributes exposing (..)
import Element.Events exposing (..)
import Form exposing (Form)
import Form.Input as Input
import Form.Validate as Validate exposing (..)
import Html
import Json.Decode as Decode exposing (..)
import Json.Decode.Pipeline exposing (..)
import Json.Encode as Encode exposing (..)
import RemoteData exposing (..)
import RemoteData.Http exposing (..)
import Return exposing (..)
import Stylesheet exposing (..)


type Msg
    = FormMsg Form.Msg
    | LoginResponse (WebData User)


type alias Model =
    { form : Form () LoginForm
    , response : WebData User
    }


type alias LoginForm =
    { email : String
    , password : String
    }


type alias User =
    { email : String
    }


init : Model
init =
    { form = Form.initial [] validation, response = NotAsked }


validation : Validation () LoginForm
validation =
    Validate.map2 LoginForm
        (Validate.field "email" Validate.email)
        (Validate.field "password" Validate.string)


userDecoder : Decoder User
userDecoder =
    decode User
        |> required "email" Decode.string


loginEncoder : LoginForm -> Encode.Value
loginEncoder loginForm =
    Encode.object
        [ ( "email", Encode.string loginForm.email )
        , ( "password", Encode.string loginForm.password )
        ]


update : Msg -> Model -> Return Msg Model
update msg model =
    case msg of
        FormMsg formMsg ->
            case ( formMsg, Form.getOutput model.form ) of
                ( Form.Submit, Just loginForm ) ->
                    return { model | response = Loading }
                        (RemoteData.Http.post "/admin/login"
                            LoginResponse
                            userDecoder
                            (loginEncoder loginForm)
                        )

                _ ->
                    return { model | form = Form.update validation formMsg model.form } Cmd.none

        LoginResponse response ->
            return { model | response = response } Cmd.none


view : Model -> Element Styles variation Msg
view model =
    column None
        []
        [ h1 Title [ paddingBottom 30 ] (text "Login")
        , loginForm model.form
        , case model.response of
            Loading ->
                text "Loading..."

            Success user ->
                text ("Signed in as " ++ user.email)

            Failure _ ->
                text "Error"

            NotAsked ->
                empty
        ]


loginForm : Form e o -> Element Styles variation Msg
loginForm form =
    let
        errorFor field =
            case field.liveError of
                Just error ->
                    text (toString error)

                Nothing ->
                    text ""

        email =
            Form.getFieldAsString "email" form

        password =
            Form.getFieldAsString "password" form
    in
    column None
        []
        [ row None
            []
            [ el None [ width (px 80) ] (text "Email")
            , html <| Html.map FormMsg (Input.textInput email [])
            ]
        , errorFor email
        , row None
            []
            [ el None [ width (px 80) ] (text "Password")
            , html <| Html.map FormMsg (Input.passwordInput password [])
            ]
        , errorFor password
        , button None
            [ onClick (FormMsg Form.Submit), width (px 200), paddingXY 0 10 ]
            (text "Enter")
        ]
