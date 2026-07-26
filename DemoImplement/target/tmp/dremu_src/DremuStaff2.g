[
    string form = "Dremu",
    string displayName = "A1"
]
@ElementStaff
class DremuStaff2
{
    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 11.10526,
            minLength : 1.263158,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period()
    {
        return new GorgeFramework.Element^[14]{
            Dremu.DremuTap : {
                laneName : "Guide1",
                hitTime : 0.0,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide2",
                hitTime : 0.2105263,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide3",
                hitTime : 0.3157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide4",
                hitTime : 0.5263158,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide5",
                hitTime : 0.8421053,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide6",
                hitTime : 0.9473684,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide7",
                hitTime : 1.157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide1",
                generateTime : -0.85,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.0,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide2",
                generateTime : -0.6394737,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.666667,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3",
                generateTime : -0.5342105,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.0,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide4",
                generateTime : -0.3236842,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : -0.6666666,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide5",
                generateTime : -0.0078947,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.333333,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide6",
                generateTime : 0.0973684,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide7",
                generateTime : 0.307895,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.333333,
                },
            },
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 12.36842,
            minLength : 1.263158,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period1()
    {
        return new GorgeFramework.Element^[14]{
            Dremu.DremuTap : {
                laneName : "Guide1",
                hitTime : 0.0,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide2",
                hitTime : 0.2105263,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide3",
                hitTime : 0.3157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide4",
                hitTime : 0.5263158,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide5",
                hitTime : 0.8421053,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide6",
                hitTime : 0.9473684,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide7",
                hitTime : 1.157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide1",
                generateTime : -0.85,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide2",
                generateTime : -0.6394737,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 5.369366,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3",
                generateTime : -0.5342105,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 6.081616,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide4",
                generateTime : -0.3236842,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 7.563086,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide5",
                generateTime : -0.0078947,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 9.933372,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide6",
                generateTime : 0.0973684,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 10.7645,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide7",
                generateTime : 0.307895,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 12.49065,
                },
            },
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 13.63158,
            minLength : 1.263158,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period2()
    {
        return new GorgeFramework.Element^[14]{
            Dremu.DremuTap : {
                laneName : "Guide1",
                hitTime : 0.0,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide2",
                hitTime : 0.2105263,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide3",
                hitTime : 0.3157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide4",
                hitTime : 0.5263158,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide5",
                hitTime : 0.8421053,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide6",
                hitTime : 0.9473684,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide7",
                hitTime : 1.157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide1",
                generateTime : -0.85,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 13.3865,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide2",
                generateTime : -0.6394737,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 15.24575,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3",
                generateTime : -0.5342105,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 16.21,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide4",
                generateTime : -0.3236842,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 18.20985,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide5",
                generateTime : -0.0078947,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 21.39456,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide6",
                generateTime : 0.0973684,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 22.50722,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide7",
                generateTime : 0.307895,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 24.81183,
                },
            },
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 14.89474,
            minLength : 1.263158,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period3()
    {
        return new GorgeFramework.Element^[14]{
            Dremu.DremuTap : {
                laneName : "Guide1",
                hitTime : 0.0,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide2",
                hitTime : 0.2105263,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide3",
                hitTime : 0.3157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide4",
                hitTime : 0.5263158,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide5",
                hitTime : 0.8421053,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide6",
                hitTime : 0.9473684,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide7",
                hitTime : 1.157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide1",
                generateTime : -0.85,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 26.00474,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide2",
                generateTime : -0.6394737,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 28.47403,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3",
                generateTime : -0.5342105,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 29.75138,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide4",
                generateTime : -0.3236842,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 32.39386,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide5",
                generateTime : -0.0078947,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 36.58448,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide6",
                generateTime : 0.0973684,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 38.04386,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide7",
                generateTime : 0.307895,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 41.05938,
                },
            },
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 19.94737,
            minLength : 1.263158,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period4()
    {
        return new GorgeFramework.Element^[16]{
            Dremu.DremuTap : {
                laneName : "Guide1",
                hitTime : 0.0,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide2",
                hitTime : 0.2105263,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide3",
                hitTime : 0.3157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide4",
                hitTime : 0.5263158,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide5",
                hitTime : 0.8421053,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide6",
                hitTime : 0.9473684,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide7",
                hitTime : 1.157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTaplik : {
                laneName : "Guide8",
                hitTime : 1.263158,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide1",
                generateTime : -0.85,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 125.5068,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide2",
                generateTime : -0.6394737,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 131.9454,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3",
                generateTime : -0.5342105,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 135.25,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide4",
                generateTime : -0.3236842,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 142.0331,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide5",
                generateTime : -0.0078947,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 152.6538,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide6",
                generateTime : 0.0973684,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 156.3158,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide7",
                generateTime : 0.307895,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : 0.85,
                        timeEnd : 1.060526,
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 163.827,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide8",
                generateTime : 0.413158,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : 0.85,
                        timeEnd : 0.9552632,
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 167.0,
                },
            },
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 18.68421,
            minLength : 1.263158,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period5()
    {
        return new GorgeFramework.Element^[14]{
            Dremu.DremuTap : {
                laneName : "Guide1",
                hitTime : 0.0,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide2",
                hitTime : 0.2105263,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide3",
                hitTime : 0.3157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide4",
                hitTime : 0.5263158,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide5",
                hitTime : 0.8421053,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide6",
                hitTime : 0.9473684,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide7",
                hitTime : 1.157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide1",
                generateTime : -0.85,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 91.36,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide2",
                generateTime : -0.6394737,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 96.54412,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3",
                generateTime : -0.5342105,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 99.2093,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide4",
                generateTime : -0.3236842,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 104.689,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide5",
                generateTime : -0.0078947,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 113.292,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide6",
                generateTime : 0.0973684,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 116.2646,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide7",
                generateTime : 0.307895,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 122.3713,
                },
            },
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 17.42105,
            minLength : 1.263158,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period6()
    {
        return new GorgeFramework.Element^[14]{
            Dremu.DremuTap : {
                laneName : "Guide1",
                hitTime : 0.0,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide2",
                hitTime : 0.2105263,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide3",
                hitTime : 0.3157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide4",
                hitTime : 0.5263158,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide5",
                hitTime : 0.8421053,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide6",
                hitTime : 0.9473684,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide7",
                hitTime : 1.157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide1",
                generateTime : -0.85,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 64.08217,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide2",
                generateTime : -0.6394737,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 68.19751,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3",
                generateTime : -0.5342105,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 70.31714,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide4",
                generateTime : -0.3236842,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 74.68316,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide5",
                generateTime : -0.0078947,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 81.55836,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide6",
                generateTime : 0.0973684,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 83.93953,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide7",
                generateTime : 0.307895,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 88.8397,
                },
            },
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 16.15789,
            minLength : 1.263158,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period7()
    {
        return new GorgeFramework.Element^[14]{
            Dremu.DremuTap : {
                laneName : "Guide1",
                hitTime : 0.0,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide2",
                hitTime : 0.2105263,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide3",
                hitTime : 0.3157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide4",
                hitTime : 0.5263158,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide5",
                hitTime : 0.8421053,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide6",
                hitTime : 0.9473684,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Guide7",
                hitTime : 1.157895,
                leadTime : 0.8,
                lagTime : 0.2,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                },
                hintReference : true,
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide1",
                generateTime : -0.85,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 42.61657,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide2",
                generateTime : -0.6394737,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 45.83243,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3",
                generateTime : -0.5342105,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 47.49217,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide4",
                generateTime : -0.3236842,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 50.91793,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide5",
                generateTime : -0.0078947,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 56.33052,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide6",
                generateTime : 0.0973684,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 58.21,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide7",
                generateTime : 0.307895,
                keepTime : 1.25,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 10.0,
                                y : -10.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 1.25,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -4.0,
                                },
                                startX : 1.25,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 8.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.85,
                            },
                            GorgeFramework.FunctionPiece : { : },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                },
                mainLaneName : "Main1",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 62.08524,
                },
            },
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 28.78947,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period8()
    {
        return new GorgeFramework.Element^[69]{
            Dremu.DremuMainLane : {
                name : "Main3",
                keepTime : 13.89474,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.8,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 12.0,
                                y : -2.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : -10.0,
                    variationCurve : GorgeFramework.LinearCurve : {
                        timeStart : 0.9473684,
                        valueStart : 10.0,
                        timeEnd : 1.263158,
                        valueEnd : 0.0,
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 10.0,
                    variationCurve : GorgeFramework.LinearCurve : {
                        timeStart : 0.9473684,
                        valueStart : -10.0,
                        timeEnd : 1.263158,
                        valueEnd : 0.0,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0,
                            g : 0.0,
                            b : 0.0,
                        },
                        GorgeFramework.ColorArgb : {
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                    },
                    progressCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : 1.0,
                                },
                                startX : 0.9473684,
                                endX : 12.0,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : 12.0,
                                    valueStart : 1.0,
                                    timeEnd : 12.1,
                                    valueEnd : 0.0,
                                },
                                startX : 12.0,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3A1",
                generateTime : 0.05,
                keepTime : 13.79474,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.CubicHermiteSpline : {
                        startWeight : 0.7,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 10.0,
                            y : -4.5,
                        },
                        endWeight : 0.0,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                mainLaneName : "Main3",
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.5,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3A2",
                generateTime : 0.05,
                keepTime : 13.79474,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.CubicHermiteSpline : {
                        startWeight : 0.7,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 10.0,
                            y : -3.0,
                        },
                        endWeight : 0.0,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                mainLaneName : "Main3",
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3A3",
                generateTime : 0.05,
                keepTime : 13.79474,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.CubicHermiteSpline : {
                        startWeight : 0.7,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 10.0,
                            y : -1.5,
                        },
                        endWeight : 0.0,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                mainLaneName : "Main3",
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.5,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3A4",
                generateTime : 0.05,
                keepTime : 13.79474,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    null,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                mainLaneName : "Main3",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3A5",
                generateTime : 0.05,
                keepTime : 13.79474,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.CubicHermiteSpline : {
                        startWeight : 0.7,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 10.0,
                            y : 1.5,
                        },
                        endWeight : 0.0,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                mainLaneName : "Main3",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.5,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3A6",
                generateTime : 0.05,
                keepTime : 13.79474,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.CubicHermiteSpline : {
                        startWeight : 0.7,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 10.0,
                            y : 3.0,
                        },
                        endWeight : 0.0,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                mainLaneName : "Main3",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.0,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide3A7",
                generateTime : 0.05,
                keepTime : 13.79474,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.CubicHermiteSpline : {
                        startWeight : 0.7,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 10.0,
                            y : 4.5,
                        },
                        endWeight : 0.0,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                mainLaneName : "Main3",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.5,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A3",
                hitTime : 1.894737,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -1.5,
                                    y : 13.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -0.6315789,
                                    y : 7.0,
                                },
                                endTangent : -1.0,
                                endWeight : 0.33333,
                            },
                            startX : (-1.0/0.0),
                            endX : -0.6315789,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -0.6315789,
                                    y : 7.0,
                                },
                                startTangent : -1.0,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.0,
                                    y : 0.0,
                                },
                                endTangent : -8.0,
                            },
                            startX : -0.6315789,
                            endX : 0.0,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.LinearFunctionCurve : {
                                k : -8.0,
                                b : 0.0,
                            },
                            startX : 0.0,
                            endX : (1.0/0.0),
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A5",
                hitTime : 2.210526,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -1.5,
                                    y : 13.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -0.6315789,
                                    y : 7.0,
                                },
                                endTangent : -1.0,
                                endWeight : 0.33333,
                            },
                            startX : (-1.0/0.0),
                            endX : -0.6315789,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -0.6315789,
                                    y : 7.0,
                                },
                                startTangent : -1.0,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.0,
                                    y : 0.0,
                                },
                                endTangent : -8.0,
                            },
                            startX : -0.6315789,
                            endX : 0.0,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.LinearFunctionCurve : {
                                k : -8.0,
                                b : 0.0,
                            },
                            startX : 0.0,
                            endX : (1.0/0.0),
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A1",
                hitTime : 2.526316,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A5",
                hitTime : 2.736842,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 2.842105,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A6",
                hitTime : 3.052632,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuHold : {
                laneName : "Guide3A3",
                hitTime : 3.368421,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.3157895,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.1052632,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startWeight : 0.8,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.3157895,
                        y : -3.0,
                    },
                    endWeight : 0.0,
                },
                holdLength : 3.157895,
            },
            Dremu.DremuTap : {
                laneName : "Guide3A7",
                hitTime : 3.789474,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A3",
                hitTime : 4.0,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A6",
                hitTime : 4.105263,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 4.31579,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuHold : {
                laneName : "Guide3A5",
                hitTime : 4.631579,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.3157895,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.1052632,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startWeight : 0.8,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.3157895,
                        y : 3.0,
                    },
                    endWeight : 0.0,
                },
                holdLength : 3.157895,
            },
            Dremu.DremuTap : {
                laneName : "Guide3A4",
                hitTime : 5.052631,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A5",
                hitTime : 5.263158,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A3",
                hitTime : 5.368421,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A4",
                hitTime : 5.578948,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuHold : {
                laneName : "Guide3A5",
                hitTime : 5.894737,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.3157895,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.1052632,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startWeight : 0.8,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.3157895,
                        y : -4.0,
                    },
                    endWeight : 0.0,
                },
                holdLength : 3.157895,
            },
            Dremu.DremuHold : {
                laneName : "Guide3A3",
                hitTime : 5.894737,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.3157895,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.1052632,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startWeight : 0.8,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.3157895,
                        y : 4.0,
                    },
                    endWeight : 0.0,
                },
                holdLength : 3.157895,
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 6.31579,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A6",
                hitTime : 6.526316,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A6",
                hitTime : 6.631579,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 6.842105,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A5",
                hitTime : 6.947369,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A3",
                hitTime : 7.157895,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A3",
                hitTime : 7.263158,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A5",
                hitTime : 7.473684,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 7.578948,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 7.789474,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A6",
                hitTime : 7.894737,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A6",
                hitTime : 8.105263,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A1",
                hitTime : 8.210526,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A1",
                hitTime : 8.421053,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A7",
                hitTime : 8.526316,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A7",
                hitTime : 8.736842,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 8.842105,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 9.052631,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A6",
                hitTime : 9.157895,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A6",
                hitTime : 9.368421,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A3",
                hitTime : 9.473684,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A3",
                hitTime : 9.684211,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A5",
                hitTime : 9.789474,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A5",
                hitTime : 10.0,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 10.10526,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A6",
                hitTime : 10.21053,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 10.31579,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A5",
                hitTime : 10.42105,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A3",
                hitTime : 10.52632,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A5",
                hitTime : 10.63158,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 10.73684,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A6",
                hitTime : 10.8421,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 10.94737,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A7",
                hitTime : 11.05263,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A1",
                hitTime : 11.1579,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A7",
                hitTime : 11.26316,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 11.36842,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A6",
                hitTime : 11.47368,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A2",
                hitTime : 11.57895,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A5",
                hitTime : 11.68421,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A3",
                hitTime : 11.78947,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Guide3A5",
                hitTime : 11.89474,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTaplik : {
                laneName : "Guide3A4",
                hitTime : 12.0,
                leadTime : 1.5,
                lagTime : 0.6,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 59.10526,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period9()
    {
        return new GorgeFramework.Element^[32]{
            Dremu.DremuMainLane : {
                name : "Main5A1",
                generateTime : 0.0,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A2",
                generateTime : 0.0263158,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A3",
                generateTime : 0.0526316,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A4",
                generateTime : 0.0789474,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A5",
                generateTime : 0.1052632,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A6",
                generateTime : 0.131579,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A7",
                generateTime : 0.1578947,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A8",
                generateTime : 0.1842105,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A9",
                generateTime : 0.2105263,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A10",
                generateTime : 0.2368421,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A11",
                generateTime : 0.2631579,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A12",
                generateTime : 0.2894737,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A13",
                generateTime : 0.3157895,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A14",
                generateTime : 0.3421053,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A15",
                generateTime : 0.368421,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main5A16",
                generateTime : 0.3947369,
                keepTime : 10.0,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.578947,
                                        y : 3.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 3.473684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 3.473684,
                                        y : -3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                },
                                startX : 3.473684,
                                endX : 4.947369,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 4.947369,
                                        y : 3.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 5.157895,
                                        y : -3.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 4.947369,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 3.473684,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 4.947369,
                            y : -720.0,
                        },
                    },
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A16",
                hitTime : 3.473684,
                leadTime : 0.9473684,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.08,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.08,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.08,
                    r : 1.0,
                    g : 1.0,
                    b : 1.0,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A15",
                hitTime : 3.578947,
                leadTime : 1.052632,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.11,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.11,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.11,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A14",
                hitTime : 3.684211,
                leadTime : 1.157895,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.14,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.14,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.14,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A13",
                hitTime : 3.789474,
                leadTime : 1.263158,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.17,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.17,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.17,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A12",
                hitTime : 3.894737,
                leadTime : 1.368421,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.2,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.2,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.2,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A11",
                hitTime : 4.0,
                leadTime : 1.473684,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.23,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.23,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.23,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A10",
                hitTime : 4.105263,
                leadTime : 1.578947,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.26,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.26,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.26,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A9",
                hitTime : 4.210526,
                leadTime : 1.684211,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.29,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.29,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.29,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A8",
                hitTime : 4.31579,
                leadTime : 1.789474,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.32,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.32,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.32,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A7",
                hitTime : 4.421052,
                leadTime : 1.894737,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.35,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.35,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.35,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A6",
                hitTime : 4.526316,
                leadTime : 2.0,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.38,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.38,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.38,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A5",
                hitTime : 4.631579,
                leadTime : 2.105263,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.41,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.41,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.41,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A4",
                hitTime : 4.736842,
                leadTime : 2.210526,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.44,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.44,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.44,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A3",
                hitTime : 4.842105,
                leadTime : 2.315789,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.47,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.47,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.47,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main5A2",
                hitTime : 4.947369,
                leadTime : 2.421053,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.5,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 0.5,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 0.5,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main5A1",
                hitTime : 5.052631,
                leadTime : 2.526316,
                lagTime : 0.6,
                distance : GorgeFramework.ConstantFunctionCurve : { : },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    r : 0.0196078,
                    g : 0.4,
                    b : 0.5529412,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 84.36842,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period10()
    {
        return new GorgeFramework.Element^[53]{
            Dremu.DremuMainLane : {
                name : "Main6",
                keepTime : 46.73684,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 20.0,
                                y : -2.5,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.5,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 20.0,
                                y : 5.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                animation : GorgeFramework.LinearCurve : {
                    timeStart : 3.789474,
                    valueStart : 0.0,
                    timeEnd : 21.47368,
                    valueEnd : 1.0,
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : -10.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 21.47368,
                                        y : 0.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 22.42105,
                                        y : 4.0,
                                    },
                                },
                                startX : (-1.0/0.0),
                                endX : 22.42105,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 22.42105,
                                        y : 4.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 22.73684,
                                        y : 3.0,
                                    },
                                },
                                startX : 22.42105,
                                endX : 22.73684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 22.73684,
                                        y : 3.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 23.36842,
                                        y : 5.0,
                                    },
                                },
                                startX : 22.73684,
                                endX : 23.36842,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.AdditionFunctionCurve : {
                                    firstFunctionCurve : GorgeFramework.PeriodicFunctionCurve : {
                                        functionCurve : GorgeFramework.PiecewiseFunctionCurve : {
                                            functionPieces : GorgeFramework.FunctionPiece^ : {
                                                GorgeFramework.FunctionPiece : {
                                                    functionCurve : GorgeFramework.CubicHermiteSpline : {
                                                        startPoint : GorgeFramework.Vector2 : {
                                                            x : 23.36842,
                                                            y : 5.0,
                                                        },
                                                        endPoint : GorgeFramework.Vector2 : {
                                                            x : 24.0,
                                                            y : 3.0,
                                                        },
                                                    },
                                                    startX : 23.36842,
                                                    endX : 24.0,
                                                },
                                                GorgeFramework.FunctionPiece : {
                                                    functionCurve : GorgeFramework.CubicHermiteSpline : {
                                                        startPoint : GorgeFramework.Vector2 : {
                                                            x : 24.0,
                                                            y : 3.0,
                                                        },
                                                        endPoint : GorgeFramework.Vector2 : {
                                                            x : 24.63158,
                                                            y : 5.0,
                                                        },
                                                    },
                                                    startX : 24.0,
                                                    endX : 24.63158,
                                                },
                                            },
                                        },
                                        startX : 23.36842,
                                        endX : 24.63158,
                                    },
                                    secondFunctionCurve : GorgeFramework.LinearCurve : {
                                        timeStart : 23.36842,
                                        valueStart : 0.0,
                                        timeEnd : 44.84211,
                                        valueEnd : 5.0,
                                    },
                                },
                                startX : 23.36842,
                                endX : 44.84211,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : 10.0,
                                },
                                startX : 44.84211,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 10.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 21.47368,
                                        y : 0.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 22.42105,
                                        y : -4.0,
                                    },
                                },
                                startX : (-1.0/0.0),
                                endX : 22.42105,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 22.42105,
                                        y : -4.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 22.73684,
                                        y : -5.0,
                                    },
                                },
                                startX : 22.42105,
                                endX : 22.73684,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 22.73684,
                                        y : -5.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 23.36842,
                                        y : -3.0,
                                    },
                                },
                                startX : 22.73684,
                                endX : 23.36842,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.AdditionFunctionCurve : {
                                    firstFunctionCurve : GorgeFramework.PeriodicFunctionCurve : {
                                        functionCurve : GorgeFramework.PiecewiseFunctionCurve : {
                                            functionPieces : GorgeFramework.FunctionPiece^ : {
                                                GorgeFramework.FunctionPiece : {
                                                    functionCurve : GorgeFramework.CubicHermiteSpline : {
                                                        startPoint : GorgeFramework.Vector2 : {
                                                            x : 23.36842,
                                                            y : -3.0,
                                                        },
                                                        endPoint : GorgeFramework.Vector2 : {
                                                            x : 24.0,
                                                            y : -5.0,
                                                        },
                                                    },
                                                    startX : 23.36842,
                                                    endX : 24.0,
                                                },
                                                GorgeFramework.FunctionPiece : {
                                                    functionCurve : GorgeFramework.CubicHermiteSpline : {
                                                        startPoint : GorgeFramework.Vector2 : {
                                                            x : 24.0,
                                                            y : -5.0,
                                                        },
                                                        endPoint : GorgeFramework.Vector2 : {
                                                            x : 24.63158,
                                                            y : -3.0,
                                                        },
                                                    },
                                                    startX : 24.0,
                                                    endX : 24.63158,
                                                },
                                            },
                                        },
                                        startX : 23.36842,
                                        endX : 24.63158,
                                    },
                                    secondFunctionCurve : GorgeFramework.LinearCurve : {
                                        timeStart : 23.36842,
                                        valueStart : 0.0,
                                        timeEnd : 44.84211,
                                        valueEnd : -5.0,
                                    },
                                },
                                startX : 23.36842,
                                endX : 45.47368,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : -10.0,
                                },
                                startX : 45.47368,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : 1.0,
                                },
                                startX : 45.47368,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                positionZ : -1.02,
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : -4.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 2.210526,
                            y : -1.5,
                        },
                        startWeight : 0.0,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 2.842105,
                            y : 0.0,
                        },
                        endWeight : 0.8,
                    },
                },
            },
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 2.842105,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -18.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                respondHintColor2 : null,
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 3.473684,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 3.789474,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 4.421052,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main6",
                hitTime : 4.631579,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 4.736842,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main6",
                hitTime : 4.947369,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 5.052631,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 6.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 5.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 6.526316,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 6.947369,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 7.263158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 7.578948,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 8.842105,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 10.10526,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 11.05263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 11.36842,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 12.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 12.63158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 13.57895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main6",
                hitTime : 13.78947,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main6",
                hitTime : 13.89474,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 14.21053,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 14.8421,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 15.1579,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 15.78947,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 16.63158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 18.21053,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 18.94737,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 19.47368,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 20.21053,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 3.789474,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.2105263,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 4.105263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.2105263,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 5.052631,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 1.157895,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 6.31579,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.2105263,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 6.631579,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.2105263,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 7.578948,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 1.157895,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 8.842105,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.2105263,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 9.157895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.2105263,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 10.10526,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.4210526,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 10.63158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 10.73684,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.5263158,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 11.36842,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.2105263,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 11.68421,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.2105263,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 12.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 0.5263158,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 12.63158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 1.157895,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 13.89474,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 1.157895,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 15.1579,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 1.157895,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 16.42105,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 1.157895,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 17.68421,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 1.157895,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 18.94737,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 1.157895,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main6",
                hitTime : 20.21053,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
                holdTime : 1.157895,
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
            },
        };
    }


}
