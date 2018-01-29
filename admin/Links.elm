module Links exposing (..)

import Data.Category as Category
import Element exposing (..)
import Element.Attributes exposing (..)
import Html exposing (td, th, tr)
import Html.Attributes as Attr
import Html.Events as Events
import Json.Decode as Decode exposing (..)
import Json.Decode.Pipeline exposing (..)
import Json.Encode as Encode exposing (..)
import Login exposing (User)
import RemoteData exposing (..)
import RemoteData.Http exposing (..)
import Return exposing (..)
import Stylesheet exposing (..)


type Msg
    = LoadLinks
    | LinksResponse (WebData (List Link))
    | VerifyLink LinkId CategoryId
    | VerifyLinkResponse (WebData ())


type alias Model =
    { linksResponse : WebData (List Link)
    , verifyLinkResponse : WebData ()
    }


type LinkId
    = LinkId Int


type CategoryId
    = CategoryId Int
    | NoCategory


type alias Link =
    { id : LinkId
    , url : Maybe String
    , title : Maybe String
    , content : Maybe String
    , categoryId : Int
    , verifiedCategoryId : Maybe Int
    , count : Int
    }


init : Model
init =
    { linksResponse = NotAsked, verifyLinkResponse = NotAsked }


linksDecoder : Decoder (List Link)
linksDecoder =
    decode Link
        |> required "id" (Decode.map LinkId Decode.int)
        |> required "url" (nullable Decode.string)
        |> required "title" (nullable Decode.string)
        |> required "content" (nullable Decode.string)
        |> required "category_id" Decode.int
        |> required "verified_category_id" (nullable Decode.int)
        |> required "count" Decode.int
        |> Decode.list


view : User -> Model -> Element Styles variation Msg
view user model =
    column None
        []
        [ row None
            [ spread, paddingBottom 30 ]
            [ h1 Title [] (text "Flagged Links")
            , el None
                [ alignBottom ]
                (case model.verifyLinkResponse of
                    Loading ->
                        text "Saving..."

                    Success _ ->
                        text "✅ Saved"

                    Failure _ ->
                        text "Error"

                    NotAsked ->
                        empty
                )
            ]
        , case model.linksResponse of
            Loading ->
                text "Loading..."

            Success links ->
                linksTable links

            Failure _ ->
                text "Error"

            NotAsked ->
                empty
        ]


linksTable : List Link -> Element Styles variation Msg
linksTable links =
    html <|
        Html.table
            [ Attr.attribute "border" "1"
            , Attr.attribute "cellpadding" "5"
            , Attr.attribute "width" "100%"
            ]
            ([ tr []
                [ th [] [ Html.text "Title or Content" ]
                , th [] [ Html.text "Popular Category" ]
                , th [] [ Html.text "Verified Category" ]
                ]
             ]
                ++ List.map linkRow links
            )


linkRow : Link -> Html.Html Msg
linkRow link =
    let
        titleLink =
            if String.contains "http" (Maybe.withDefault "" link.url) then
                Html.a
                    [ Attr.href (Maybe.withDefault "" link.url)
                    , Attr.target "_blank"
                    ]
                    [ Html.text <| Maybe.withDefault "" link.title ]
            else
                Html.div
                    [ Attr.style [ ( "max-height", "50px" ), ( "overflow-y", "scroll" ) ]
                    ]
                    [ Html.text <| Maybe.withDefault "" link.content ]

        category =
            Category.fromId link.categoryId

        popularCategory =
            Html.text
                (Category.toEmoji category
                    ++ " "
                    ++ Category.toName category
                )

        verifyLinkEvent targetValue =
            String.toInt targetValue
                |> Result.map CategoryId
                |> Result.withDefault NoCategory
                |> VerifyLink link.id

        selectCategory =
            Html.select
                [ Attr.style [ ( "width", "100%" ) ]
                , Events.on "change" (Decode.map verifyLinkEvent Events.targetValue)
                ]
                ([ Html.option
                    [ Attr.value ""
                    , Attr.selected (link.verifiedCategoryId == Nothing)
                    ]
                    [ Html.text "" ]
                 ]
                    ++ List.map
                        (\id ->
                            Html.option
                                [ Attr.value <| toString id
                                , Attr.selected (link.verifiedCategoryId == Just id)
                                ]
                                [ Html.text (Category.toName <| Category.fromId id) ]
                        )
                        (List.range 1 6)
                )
    in
    tr []
        [ td [] [ titleLink ]
        , td [] [ popularCategory ]
        , td [] [ selectCategory ]
        ]


update : Msg -> Model -> Return Msg Model
update msg model =
    case msg of
        LoadLinks ->
            return { model | linksResponse = Loading }
                (RemoteData.Http.get "/links/all" LinksResponse linksDecoder)

        LinksResponse linksResponse ->
            return { model | linksResponse = linksResponse } Cmd.none

        VerifyLink linkId categoryId ->
            return { model | verifyLinkResponse = Loading }
                (RemoteData.Http.post
                    "/admin/verify_link"
                    VerifyLinkResponse
                    (Decode.succeed ())
                    (verifyLinkEncoder linkId categoryId)
                )

        VerifyLinkResponse verifyLinkResponse ->
            return { model | verifyLinkResponse = verifyLinkResponse } Cmd.none


verifyLinkEncoder : LinkId -> CategoryId -> Encode.Value
verifyLinkEncoder (LinkId linkId) categoryId =
    Encode.object
        [ ( "link_id", Encode.int linkId )
        , ( "category_id"
          , case categoryId of
                NoCategory ->
                    Encode.null

                CategoryId categoryId ->
                    Encode.int categoryId
          )
        ]
