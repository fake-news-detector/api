module Stylesheet exposing (..)

import Color
import Style exposing (..)
import Style.Color as Color
import Style.Font as Font


type Styles
    = None
    | General
    | Logo
    | Navigation
    | Title


stylesheet : StyleSheet Styles variation
stylesheet =
    Style.styleSheet
        [ style None []
        , style General
            [ -- source: https://www.smashingmagazine.com/2015/11/using-system-ui-fonts-practical-guide/
              Font.typeface (List.map Font.font [ "-apple-system", "BlinkMacSystemFont", "Segoe UI", "Roboto", "Oxygen", "Ubuntu", "sans-serif" ])
            , Font.size 12
            ]
        , style Logo
            [ Font.size 18
            , Color.text Color.white
            ]
        , style Navigation
            [ Color.background Color.black
            ]
        , style Title
            [ Font.size 32
            ]
        ]
