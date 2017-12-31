module Links exposing (..)

import Data.Category as Category
import Element exposing (..)
import Element.Attributes exposing (..)
import Html exposing (td, tr)
import Html.Attributes as Attr exposing (attribute)
import Json.Decode as Decode exposing (..)
import Json.Decode.Pipeline exposing (..)
import Login exposing (User)
import RemoteData exposing (..)
import RemoteData.Http exposing (..)
import Return exposing (..)
import Stylesheet exposing (..)


type Msg
    = LoadLinks
    | LinksResponse (WebData (List Link))


type alias Model =
    { response : WebData (List Link)
    }


type alias Link =
    { id : Int
    , url : Maybe String
    , title : Maybe String
    , content : Maybe String
    , categoryId : Int
    , count : Int
    }


init : Model
init =
    { response = NotAsked }


linksDecoder : Decoder (List Link)
linksDecoder =
    decode Link
        |> required "id" Decode.int
        |> required "url" (nullable Decode.string)
        |> required "title" (nullable Decode.string)
        |> required "content" (nullable Decode.string)
        |> required "category_id" Decode.int
        |> required "count" Decode.int
        |> Decode.list


view : User -> Model -> Element Styles variation msg
view user model =
    column None
        []
        [ h1 Title [ paddingBottom 30 ] (text "Flagged Links")
        , case model.response of
            Loading ->
                text "Loading..."

            Success links ->
                linksTable links

            Failure _ ->
                text "Error"

            NotAsked ->
                empty
        ]


linksTable : List Link -> Element Styles variation msg
linksTable links =
    html <|
        Html.table
            [ Attr.attribute "border" "1"
            , Attr.attribute "cellpadding" "5"
            , Attr.attribute "width" "100%"
            ]
            (List.map linkRow links)


linkRow : Link -> Html.Html msg
linkRow link =
    let
        title =
            Html.text <| Maybe.withDefault "" link.title

        titleLink =
            Html.a
                [ Attr.href (Maybe.withDefault "" link.url)
                , Attr.target "_blank"
                ]
                [ title ]

        category =
            Category.fromId link.categoryId

        categoryText =
            Html.text
                (Category.toEmoji category
                    ++ " "
                    ++ Category.toName category
                )
    in
    tr []
        [ td [] [ titleLink ]
        , td [] [ categoryText ]
        ]


update : Msg -> Model -> Return Msg Model
update msg model =
    case msg of
        LoadLinks ->
            return { model | response = Loading }
                (RemoteData.Http.get "/links/all" LinksResponse linksDecoder)

        LinksResponse response ->
            return { model | response = response } Cmd.none
