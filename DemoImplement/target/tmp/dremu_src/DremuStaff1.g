[
    string form = "Dremu",
    string displayName = "Dremu谱表"
]
@ElementStaff
class DremuStaff1
{
    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 0.0,
            minLength : 10.0,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period()
    {
        return new GorgeFramework.Element^[2]{
            Dremu.DremuMainLane : {
                name : "Main1",
                keepTime : 23.47368,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 10.0,
                            y : 0.0,
                        },
                        startWeight : 0.5,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 174.0,
                            y : -25.0,
                        },
                        endTangent : 0.0,
                        endWeight : 0.0,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : -10.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : 10.47368,
                                    valueStart : 10.0,
                                    timeEnd : 10.63158,
                                    valueEnd : 0.0,
                                },
                                startX : (-1.0/0.0),
                                endX : 11.10526,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 11.10526,
                                        y : 0.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 21.21053,
                                        y : 163.827,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.0,
                                },
                                startX : 11.10526,
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
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : 10.47368,
                                    valueStart : -10.0,
                                    timeEnd : 10.63158,
                                    valueEnd : 0.0,
                                },
                                startX : (-1.0/0.0),
                                endX : 11.10526,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 11.10526,
                                        y : 0.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 21.21053,
                                        y : 163.827,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.0,
                                },
                                startX : 11.10526,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                pointCount : 200,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : { : },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : 1.0,
                                },
                                startX : 10.47368,
                                endX : 21.21053,
                            },
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 11.10526,
                            y : 0.0,
                        },
                        startWeight : 0.5,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 21.21053,
                            y : -163.827,
                        },
                        endWeight : 0.0,
                    },
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : -0.4,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 14.0,
                            y : 0.0,
                        },
                        startWeight : 0.65,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 21.21053,
                            y : 23.0,
                        },
                        endWeight : 0.0,
                    },
                },
            },
            Dremu.DremuTaplik : {
                laneName : "Main1",
                hitTime : 10.47368,
                leadTime : 1.263158,
                distance : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -0.8315789,
                                    y : 0.0,
                                },
                                startTangent : -3.0,
                                startWeight : 0.7,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -0.1,
                                    y : 3.0,
                                },
                                endTangent : 0.0,
                                endWeight : 0.8,
                            },
                            startX : -0.8315789,
                            endX : -0.1,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -0.1,
                                    y : 3.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.0,
                                    y : 0.0,
                                },
                                endTangent : 0.0,
                                endWeight : 0.0,
                            },
                            startX : -0.1,
                            endX : 0.0,
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                        GorgeFramework.ColorArgb : {
                            r : 0.9411765,
                            g : 0.9490196,
                            b : 0.7568628,
                        },
                    },
                    progressCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : -1.263158,
                                    timeEnd : -0.9473684,
                                },
                                startX : -10.0,
                                endX : 0.0,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : 0.0,
                                    valueStart : 1.0,
                                    timeEnd : 0.2,
                                    valueEnd : 0.0,
                                },
                            },
                        },
                    },
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
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
            timeOffset : 21.21053,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period1()
    {
        return new GorgeFramework.Element^[40]{
            Dremu.DremuMainLane : {
                name : "Main2R",
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            endPoint : GorgeFramework.Vector2 : {
                                x : 8.0,
                                y : 2.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.0,
                                        y : -0.45,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 0.5263158,
                                        y : -3.5,
                                    },
                                    endTangent : -2.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.5263158,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.5263158,
                                        y : -3.5,
                                    },
                                    startTangent : -2.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 0.8421053,
                                        y : -4.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : 0.5263158,
                                endX : 7.894737,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 7.894737,
                                        y : -4.0,
                                    },
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 8.421053,
                                        y : -1.333333,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 7.894737,
                                endX : 9.157895,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 9.157895,
                                        y : -1.333333,
                                    },
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 9.684211,
                                        y : 0.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 9.157895,
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
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.2,
                                        y : -0.45,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 0.5263158,
                                        y : -2.0,
                                    },
                                    endTangent : -2.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.5263158,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.5263158,
                                        y : -2.0,
                                    },
                                    startTangent : -2.0,
                                    startWeight : 0.8,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 0.8421053,
                                        y : -2.0,
                                    },
                                    endTangent : -2.0,
                                    endWeight : 0.2,
                                },
                                startX : 0.5263158,
                                endX : 0.8421053,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.8421053,
                                        y : -2.0,
                                    },
                                    startTangent : 2.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 1.157895,
                                        y : 4.0,
                                    },
                                },
                                startX : 0.8421053,
                                endX : 8.526316,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 8.526316,
                                        y : 4.0,
                                    },
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 9.052631,
                                        y : 1.333333,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 8.526316,
                                endX : 9.157895,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 9.157895,
                                        y : 1.333333,
                                    },
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 9.684211,
                                        y : 0.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 9.157895,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                pointCount : 200,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                    progressCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : { : },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : 1.0,
                                },
                                startX : 9.684211,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : 3.25,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.71,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.157895,
                            y : 2.25,
                        },
                    },
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 80.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.71,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.157895,
                            y : 10.0,
                        },
                    },
                },
            },
            Dremu.DremuMainLane : {
                name : "Main2L",
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            endPoint : GorgeFramework.Vector2 : {
                                x : 8.0,
                                y : 2.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.0,
                                        y : 5.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 0.5263158,
                                        y : -3.5,
                                    },
                                    endTangent : -2.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.5263158,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.5263158,
                                        y : -3.5,
                                    },
                                    startTangent : -2.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 0.8421053,
                                        y : -4.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : 0.5263158,
                                endX : 7.578948,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 7.578948,
                                        y : -4.0,
                                    },
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 8.105263,
                                        y : -1.333333,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 7.578948,
                                endX : 8.842105,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 8.842105,
                                        y : -1.333333,
                                    },
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 9.368421,
                                        y : 0.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 8.842105,
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
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.2,
                                        y : 5.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 0.5263158,
                                        y : -2.0,
                                    },
                                    endTangent : -2.0,
                                    endWeight : 0.5,
                                },
                                startX : (-1.0/0.0),
                                endX : 0.5263158,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.5263158,
                                        y : -2.0,
                                    },
                                    startTangent : -2.0,
                                    startWeight : 0.8,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 0.8421053,
                                        y : -2.0,
                                    },
                                    endTangent : -2.0,
                                    endWeight : 0.2,
                                },
                                startX : 0.5263158,
                                endX : 0.8421053,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.8421053,
                                        y : -2.0,
                                    },
                                    startTangent : 2.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 1.157895,
                                        y : 4.0,
                                    },
                                },
                                startX : 0.8421053,
                                endX : 8.210526,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 8.210526,
                                        y : 4.0,
                                    },
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 8.736842,
                                        y : 1.333333,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 8.210526,
                                endX : 8.842105,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 8.842105,
                                        y : 1.333333,
                                    },
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 9.368421,
                                        y : 0.0,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 8.842105,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                pointCount : 200,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.0196078,
                            g : 0.4,
                            b : 0.5529412,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                    progressCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : { : },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : 1.0,
                                },
                                startX : 9.368421,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                positionX : GorgeFramework.VariableFloat : {
                    baseValue : -3.25,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.71,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.157895,
                            y : -2.25,
                        },
                        endTangent : 0.0,
                    },
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 260.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.71,
                            y : 0.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.157895,
                            y : 10.0,
                        },
                    },
                },
            },
            Dremu.DremuTap : {
                laneName : "Main2L",
                hitTime : 0.8421053,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
            Dremu.DremuDrag : {
                laneName : "Main2L",
                hitTime : 0.9473684,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
            Dremu.DremuDrag : {
                laneName : "Main2L",
                hitTime : 1.157895,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2R",
                hitTime : 1.263158,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2L",
                hitTime : 1.473684,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2R",
                hitTime : 1.578947,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2L",
                hitTime : 1.789474,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2R",
                hitTime : 2.105263,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
            Dremu.DremuDrag : {
                laneName : "Main2R",
                hitTime : 2.210526,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
            Dremu.DremuDrag : {
                laneName : "Main2R",
                hitTime : 2.421053,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2L",
                hitTime : 2.526316,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2R",
                hitTime : 2.736842,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2L",
                hitTime : 2.842105,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2R",
                hitTime : 3.052632,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2L",
                hitTime : 3.368421,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
            Dremu.DremuDrag : {
                laneName : "Main2L",
                hitTime : 3.473684,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
            Dremu.DremuDrag : {
                laneName : "Main2L",
                hitTime : 3.684211,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2R",
                hitTime : 3.789474,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2L",
                hitTime : 4.0,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2R",
                hitTime : 4.105263,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2L",
                hitTime : 4.31579,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2R",
                hitTime : 4.631579,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
            Dremu.DremuDrag : {
                laneName : "Main2R",
                hitTime : 4.736842,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
            Dremu.DremuDrag : {
                laneName : "Main2R",
                hitTime : 4.947369,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2L",
                hitTime : 5.052631,
                leadTime : 0.6,
                lagTime : 0.6,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                holdTime : 0.5263158,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.5263158,
                    },
                },
                endDistance : GorgeFramework.LinearCurve : {
                    timeStart : -0.6,
                    valueStart : 4.8,
                    timeEnd : 0.0,
                    valueEnd : 0.0,
                },
                holdLine : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : null,
                            startX : 0.0,
                            endX : 0.0,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.LinearCurve : {
                                timeStart : 0.0,
                                timeEnd : 0.3157895,
                                valueEnd : 1.0,
                            },
                            startX : 0.0,
                            endX : 0.3157895,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.LinearCurve : {
                                timeStart : 0.3157895,
                                valueStart : 1.0,
                                timeEnd : 0.5263158,
                                valueEnd : -1.0,
                            },
                            startX : 0.3157895,
                            endX : (1.0/0.0),
                        },
                    },
                },
                holdLength : 4.210526,
            },
            Dremu.DremuTap : {
                laneName : "Main2R",
                hitTime : 5.894737,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2R",
                hitTime : 6.0,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2L",
                hitTime : 6.210526,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2R",
                hitTime : 6.31579,
                leadTime : 0.6,
                lagTime : 0.6,
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                holdTime : 0.5263158,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.2105263,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.5263158,
                    },
                },
                endDistance : GorgeFramework.LinearCurve : {
                    timeStart : -0.6,
                    valueStart : 4.8,
                    timeEnd : 0.0,
                    valueEnd : 0.0,
                },
                holdLine : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : null,
                            startX : 0.0,
                            endX : 0.0,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.LinearCurve : {
                                timeStart : 0.0,
                                timeEnd : 0.2105263,
                                valueEnd : 1.0,
                            },
                            startX : 0.0,
                            endX : 0.2105263,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.LinearCurve : {
                                timeStart : 0.2105263,
                                valueStart : 1.0,
                                timeEnd : 0.3157895,
                                valueEnd : -1.0,
                            },
                            startX : 0.2105263,
                            endX : 0.3157895,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.LinearCurve : {
                                timeStart : 0.3157895,
                                valueStart : -1.0,
                                timeEnd : 0.5263158,
                                valueEnd : 0.0,
                            },
                            startX : 0.3157895,
                            endX : (1.0/0.0),
                        },
                    },
                },
                holdLength : 4.210526,
            },
            Dremu.DremuTap : {
                laneName : "Main2L",
                hitTime : 7.157895,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2L",
                hitTime : 7.263158,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2R",
                hitTime : 7.473684,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                laneName : "Main2L",
                hitTime : 7.578948,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                holdTime : 0.2105263,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.2105263,
                    },
                },
                endDistance : GorgeFramework.LinearCurve : {
                    timeStart : -0.6,
                    valueStart : 4.8,
                    timeEnd : 0.0,
                    valueEnd : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startWeight : 0.0,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.2105263,
                        y : 2.0,
                    },
                    endWeight : 0.6,
                },
                holdLength : 4.210526,
            },
            Dremu.DremuHold : {
                laneName : "Main2R",
                hitTime : 7.894737,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                holdTime : 0.2105263,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.2105263,
                    },
                },
                endDistance : GorgeFramework.LinearCurve : {
                    timeStart : -0.6,
                    valueStart : 4.8,
                    timeEnd : 0.0,
                    valueEnd : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startWeight : 0.0,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.2105263,
                        y : 2.0,
                    },
                    endWeight : 0.6,
                },
                holdLength : 4.210526,
            },
            Dremu.DremuHold : {
                laneName : "Main2L",
                hitTime : 8.210526,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.5,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                holdTime : 0.2105263,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.2105263,
                    },
                },
                endDistance : GorgeFramework.LinearCurve : {
                    timeStart : -0.6,
                    valueStart : 4.8,
                    timeEnd : 0.0,
                    valueEnd : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startWeight : 0.0,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.2105263,
                        y : -2.0,
                    },
                    endWeight : 0.6,
                },
                holdLength : 4.210526,
            },
            Dremu.DremuHold : {
                laneName : "Main2R",
                hitTime : 8.526316,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.5,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                holdTime : 0.2105263,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.2105263,
                    },
                },
                endDistance : GorgeFramework.LinearCurve : {
                    timeStart : -0.6,
                    valueStart : 4.8,
                    timeEnd : 0.0,
                    valueEnd : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startWeight : 0.0,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.2105263,
                        y : -2.0,
                    },
                    endWeight : 0.6,
                },
                holdLength : 4.210526,
            },
            Dremu.DremuHold : {
                laneName : "Main2L",
                hitTime : 8.842105,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                holdTime : 0.2105263,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.2105263,
                    },
                },
                endDistance : GorgeFramework.LinearCurve : {
                    timeStart : -0.6,
                    valueStart : 4.8,
                    timeEnd : 0.0,
                    valueEnd : 0.0,
                },
                holdLength : 4.210526,
            },
            Dremu.DremuHold : {
                laneName : "Main2R",
                hitTime : 9.157895,
                leadTime : 0.6,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                        GorgeFramework.ColorArgb : {
                            a : 1.0,
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : GorgeFramework.LinearCurve : {
                        timeStart : -0.6,
                        timeEnd : -0.3,
                    },
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
                holdTime : 0.2105263,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.2105263,
                    },
                },
                endDistance : GorgeFramework.LinearCurve : {
                    timeStart : -0.6,
                    valueStart : 4.8,
                    timeEnd : 0.0,
                    valueEnd : 0.0,
                },
                holdLength : 4.210526,
            },
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 40.15789,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period2()
    {
        return new GorgeFramework.Element^[109]{
            Dremu.DremuMainLane : {
                name : "Main4",
                keepTime : 48.0,
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : -10.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : 1.263158,
                                    valueStart : 10.0,
                                    timeEnd : 1.473684,
                                    valueEnd : 0.0,
                                },
                                startX : (-1.0/0.0),
                                endX : 43.57895,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : -10.0,
                                    b : 435.7895,
                                },
                                startX : 43.57895,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 10.0,
                    variationCurve : GorgeFramework.LinearCurve : {
                        timeStart : 1.263158,
                        valueStart : -10.0,
                        timeEnd : 1.473684,
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
                                startX : 1.263158,
                                endX : 21.47368,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : null,
                                startX : 21.47368,
                                endX : 24.0,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : 1.0,
                                },
                                startX : 24.0,
                                endX : 46.42105,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                    value : 0.0,
                                },
                                startX : 46.42105,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 18.94737,
                                        y : 0.0,
                                    },
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 20.84211,
                                        y : 3.5,
                                    },
                                    endWeight : 0.5,
                                },
                                startX : 0.0,
                                endX : 43.57895,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                    k : 10.0,
                                    b : -432.2895,
                                },
                                startX : 43.57895,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 29.05263,
                                        y : 0.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 29.68421,
                                        y : -5.0,
                                    },
                                },
                                startX : (-1.0/0.0),
                                endX : 29.68421,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 29.68421,
                                        y : -5.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 30.31579,
                                        y : 5.0,
                                    },
                                },
                                startX : 29.68421,
                                endX : 30.31579,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 30.31579,
                                        y : 5.0,
                                    },
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 30.94737,
                                        y : -5.0,
                                    },
                                },
                                startX : 30.31579,
                                endX : 30.94737,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 30.94737,
                                        y : -5.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 31.57895,
                                        y : 5.0,
                                    },
                                    endTangent : 20.0,
                                    endWeight : 0.1,
                                },
                                startX : 30.94737,
                                endX : 31.57895,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 31.57895,
                                        y : 5.0,
                                    },
                                    startTangent : 20.0,
                                    startWeight : 0.1,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 33.78947,
                                        y : 200.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : 31.57895,
                                endX : 33.78947,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 33.78947,
                                        y : 200.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 36.63158,
                                        y : 180.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : 33.78947,
                                endX : 37.57895,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 37.57895,
                                        y : 180.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 37.89474,
                                        y : 175.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.8,
                                },
                                startX : 37.57895,
                                endX : 37.89474,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 37.89474,
                                        y : 175.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 38.21053,
                                        y : 185.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.8,
                                },
                                startX : 37.89474,
                                endX : 38.84211,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 38.84211,
                                        y : 185.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 39.15789,
                                        y : 175.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.8,
                                },
                                startX : 38.84211,
                                endX : 39.78947,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 39.78947,
                                        y : 175.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.0,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 41.05263,
                                        y : 190.0,
                                    },
                                    endTangent : 1.0,
                                    endWeight : 0.8,
                                },
                                startX : 39.78947,
                                endX : 41.05263,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 41.05263,
                                        y : 190.0,
                                    },
                                    startTangent : 1.0,
                                    startWeight : 0.8,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 41.68421,
                                        y : 370.0,
                                    },
                                    endTangent : 20.0,
                                    endWeight : 0.3,
                                },
                                startX : 41.05263,
                                endX : 41.68421,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 41.68421,
                                        y : 370.0,
                                    },
                                    startTangent : 20.0,
                                    startWeight : 0.3,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 42.31579,
                                        y : 370.0,
                                    },
                                    endTangent : 20.0,
                                    endWeight : 0.3,
                                },
                                startX : 41.68421,
                                endX : 42.31579,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 42.31579,
                                        y : 370.0,
                                    },
                                    startTangent : 20.0,
                                    startWeight : 0.8,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 42.94737,
                                        y : 170.0,
                                    },
                                    endTangent : -300.0,
                                    endWeight : 0.3,
                                },
                                startX : 42.31579,
                                endX : 42.94737,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 42.94737,
                                        y : 170.0,
                                    },
                                    startTangent : -300.0,
                                    startWeight : 0.3,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 43.57895,
                                        y : 90.0,
                                    },
                                    endTangent : -20.0,
                                    endWeight : 0.3,
                                },
                                startX : 42.94737,
                                endX : 43.57895,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 43.57895,
                                        y : 90.0,
                                    },
                                    startTangent : -20.0,
                                    startWeight : 0.3,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 44.21053,
                                        y : 90.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.3,
                                },
                                startX : 43.57895,
                                endX : (1.0/0.0),
                            },
                        },
                    },
                },
            },
            Dremu.DremuTaplik : {
                laneName : "Main4",
                hitTime : 1.263158,
                leadTime : 0.6315789,
                distance : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -0.6315789,
                                    y : 0.0,
                                },
                                startWeight : 0.0,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -0.3157895,
                                    y : 3.5,
                                },
                                endWeight : 0.8,
                            },
                            startX : (-1.0/0.0),
                            endX : -0.3157895,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -0.3157895,
                                    y : 3.5,
                                },
                                startWeight : 0.8,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.0,
                                    y : 0.0,
                                },
                                endWeight : 0.0,
                            },
                            startX : -0.3157895,
                            endX : (1.0/0.0),
                        },
                    },
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
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 1.894737,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -15.0,
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
                holdTime : 0.6315789,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.6315789,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -14.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startWeight : 0.0,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.3157895,
                        y : 7.0,
                    },
                    endWeight : 0.5,
                },
                holdLength : 9.473683,
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 2.210526,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -16.0,
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
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 2.526316,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -14.0,
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
                holdTime : 0.6315789,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.6315789,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startWeight : 0.0,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.3157895,
                        y : 7.0,
                    },
                    endWeight : 0.5,
                },
                holdLength : 8.842105,
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 2.842105,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -14.0,
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
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 3.157895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
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
                holdTime : 0.3157895,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startWeight : 0.0,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.3157895,
                        y : 7.0,
                    },
                    endWeight : 0.5,
                },
                holdLength : 3.789474,
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 3.473684,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 3.789474,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 4.105263,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 4.263158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 4.736842,
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
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 4.842105,
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
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 4.947369,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 5.052631,
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
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 5.052631,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 5.368421,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 5.368421,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 5.68421,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
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
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 5.789474,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -0.5,
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
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 5.894737,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.5,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 6.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.5,
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
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 6.105263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.5,
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
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 6.210526,
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
            Dremu.DremuHold : {
                laneName : "Guide4A",
                hitTime : 6.31579,
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
                holdTime : 5.052631,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.6315789,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 5.052631,
                    },
                },
                endDistance : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.LinearFunctionCurve : {
                                k : -12.0,
                                b : -30.31579,
                            },
                            startX : (-1.0/0.0),
                            endX : -2.842105,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -2.842105,
                                    y : 3.789474,
                                },
                                startTangent : -12.0,
                                startWeight : 0.5,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -2.684211,
                                    y : 1.0,
                                },
                                endTangent : 0.0,
                                endWeight : 0.0,
                            },
                            startX : -2.842105,
                            endX : -2.684211,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -2.684211,
                                    y : 1.0,
                                },
                                startTangent : 20.0,
                                startWeight : 0.5,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -2.526316,
                                    y : 3.0,
                                },
                                endTangent : 2.0,
                                endWeight : 0.33333,
                            },
                            startX : -2.684211,
                            endX : -2.526316,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -2.526316,
                                    y : 3.0,
                                },
                                startTangent : 2.0,
                                startWeight : 0.33333,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -2.210526,
                                    y : 3.0,
                                },
                                endTangent : 2.0,
                                endWeight : 0.33333,
                            },
                            startX : -2.526316,
                            endX : -2.210526,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -2.210526,
                                    y : 3.0,
                                },
                                startTangent : 2.0,
                                startWeight : 0.33333,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -2.052632,
                                    y : 1.0,
                                },
                                endTangent : -20.0,
                                endWeight : 0.33333,
                            },
                            startX : -2.210526,
                            endX : -2.052632,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -2.052632,
                                    y : 1.0,
                                },
                                startTangent : 20.0,
                                startWeight : 0.5,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -1.894737,
                                    y : 3.0,
                                },
                                endTangent : 2.0,
                                endWeight : 0.33333,
                            },
                            startX : -2.052632,
                            endX : -1.894737,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -1.894737,
                                    y : 3.0,
                                },
                                startTangent : 2.0,
                                startWeight : 0.33333,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -1.578947,
                                    y : 3.0,
                                },
                                endTangent : 2.0,
                                endWeight : 0.33333,
                            },
                            startX : -1.894737,
                            endX : -1.578947,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -1.578947,
                                    y : 3.0,
                                },
                                startTangent : 2.0,
                                startWeight : 0.33333,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -1.421053,
                                    y : 1.0,
                                },
                                endTangent : -20.0,
                                endWeight : 0.33333,
                            },
                            startX : -1.578947,
                            endX : -1.421053,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -1.421053,
                                    y : 1.0,
                                },
                                startTangent : 20.0,
                                startWeight : 0.5,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -1.263158,
                                    y : 3.0,
                                },
                                endTangent : 2.0,
                                endWeight : 0.33333,
                            },
                            startX : -1.421053,
                            endX : -1.263158,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -1.263158,
                                    y : 3.0,
                                },
                                startTangent : 2.0,
                                startWeight : 0.33333,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -0.9473684,
                                    y : 3.0,
                                },
                                endTangent : 2.0,
                                endWeight : 0.33333,
                            },
                            startX : -1.263158,
                            endX : -0.9473684,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -0.9473684,
                                    y : 3.0,
                                },
                                startTangent : 2.0,
                                startWeight : 0.33333,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -0.7894737,
                                    y : 1.0,
                                },
                                endTangent : -20.0,
                                endWeight : 0.33333,
                            },
                            startX : -0.9473684,
                            endX : -0.7894737,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -0.7894737,
                                    y : 1.0,
                                },
                                startTangent : 20.0,
                                startWeight : 0.5,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -0.6315789,
                                    y : 3.0,
                                },
                                endTangent : 2.0,
                                endWeight : 0.33333,
                            },
                            startX : -0.7894737,
                            endX : -0.6315789,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -0.6315789,
                                    y : 3.0,
                                },
                                startTangent : 2.0,
                                startWeight : 0.33333,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -0.3157895,
                                    y : 3.0,
                                },
                                endTangent : 2.0,
                                endWeight : 0.33333,
                            },
                            startX : -0.6315789,
                            endX : -0.3157895,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -0.3157895,
                                    y : 3.0,
                                },
                                startTangent : -2.0,
                                startWeight : 0.33333,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.0,
                                    y : 0.0,
                                },
                                endTangent : 0.0,
                                endWeight : 0.0,
                            },
                            startX : -0.3157895,
                            endX : 0.0,
                        },
                    },
                },
                holdLine : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.ConstantFunctionCurve : { : },
                            startX : 0.0,
                            endX : 0.9473684,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.9473684,
                                    y : 0.0,
                                },
                                startWeight : 0.5,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.263158,
                                    y : 3.0,
                                },
                                endTangent : 20.0,
                                endWeight : 0.2,
                            },
                            startX : 0.9473684,
                            endX : 1.263158,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 1.263158,
                                    y : 3.0,
                                },
                                startTangent : 20.0,
                                startWeight : 0.2,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.578947,
                                    y : 3.0,
                                },
                                endTangent : 20.0,
                                endWeight : 0.2,
                            },
                            startX : 1.263158,
                            endX : 1.578947,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 1.578947,
                                    y : 3.0,
                                },
                                startTangent : -20.0,
                                startWeight : -0.2,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.894737,
                                    y : 0.0,
                                },
                                endTangent : 0.0,
                                endWeight : 0.5,
                            },
                            startX : 1.578947,
                            endX : 1.894737,
                        },
                    },
                },
                holdLength : 60.63157,
                pointCount : 200,
            },
            Dremu.DremuGuideLane : {
                name : "Guide4A",
                generateTime : 3.789474,
                keepTime : 10.10526,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.ConstantFunctionCurve : {
                        value : 0.0,
                    },
                    GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 1.0,
                            y : 0.0,
                        },
                        startWeight : 0.5,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 3.5,
                            y : -2.5,
                        },
                        endTangent : 0.0,
                        endWeight : 0.0,
                    },
                    GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 1.0,
                            y : 0.0,
                        },
                        startWeight : 0.5,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 3.5,
                            y : 2.5,
                        },
                        endTangent : 0.0,
                        endWeight : 0.0,
                    },
                },
                animation : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            startX : 0.0,
                            endX : 4.894737,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                value : 1.0,
                            },
                            startX : 4.894737,
                            endX : 5.526316,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                value : 2.0,
                            },
                            startX : 5.526316,
                            endX : 6.157895,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                value : 1.0,
                            },
                            startX : 6.157895,
                            endX : 6.789474,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                value : 2.0,
                            },
                            startX : 6.789474,
                            endX : (1.0/0.0),
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                mainLaneName : "Main4",
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide4B1",
                generateTime : 30.31579,
                keepTime : 10.0,
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
                                y : -20.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                mainLaneName : "Main4",
                position : GorgeFramework.VariableFloat : {
                    baseValue : -5.0,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide4B2",
                generateTime : 30.31579,
                keepTime : 10.0,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.8,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 20.0,
                                y : -20.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                mainLaneName : "Main4",
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide4B3",
                generateTime : 30.31579,
                keepTime : 10.0,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.AxialSymmetricFunctionCurve : {
                        functionCurve : GorgeFramework.CubicHermiteSpline : {
                            startPoint : GorgeFramework.Vector2 : {
                                x : 0.0,
                                y : 0.0,
                            },
                            startWeight : 0.8,
                            endPoint : GorgeFramework.Vector2 : {
                                x : 20.0,
                                y : 20.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                mainLaneName : "Main4",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.5,
                },
            },
            Dremu.DremuGuideLane : {
                name : "Guide4B4",
                generateTime : 30.31579,
                keepTime : 10.0,
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
                                y : 20.0,
                            },
                            endWeight : 0.0,
                        },
                        keepLeft : false,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            a : 0.0,
                        },
                    },
                },
                mainLaneName : "Main4",
                position : GorgeFramework.VariableFloat : {
                    baseValue : 5.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 7.263158,
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
                holdTime : 1.473684,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.1052632,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.2105263,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.6315789,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.9473684,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.052632,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.263158,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.368421,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.473684,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.0,
                                    y : 0.0,
                                },
                                startWeight : 0.5,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.3157895,
                                    y : -3.0,
                                },
                                endTangent : -20.0,
                                endWeight : 0.2,
                            },
                            startX : 0.0,
                            endX : 0.3157895,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.3157895,
                                    y : -3.0,
                                },
                                startTangent : -20.0,
                                startWeight : 0.2,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.6315789,
                                    y : -3.0,
                                },
                                endTangent : -20.0,
                                endWeight : 0.2,
                            },
                            startX : 0.3157895,
                            endX : 0.6315789,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.6315789,
                                    y : -3.0,
                                },
                                startTangent : 20.0,
                                startWeight : -0.2,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.9473684,
                                    y : 0.0,
                                },
                                endTangent : 0.0,
                                endWeight : 0.5,
                            },
                            startX : 0.6315789,
                            endX : 0.9473684,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.9473684,
                                    y : 0.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.052632,
                                    y : -0.5,
                                },
                            },
                            startX : 0.9473684,
                            endX : 1.052632,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 1.052632,
                                    y : -0.5,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.157895,
                                    y : 1.0,
                                },
                            },
                            startX : 1.052632,
                            endX : 1.157895,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 1.157895,
                                    y : 1.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.263158,
                                    y : -1.5,
                                },
                            },
                            startX : 1.157895,
                            endX : 1.263158,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 1.263158,
                                    y : -1.5,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.368421,
                                    y : 1.5,
                                },
                            },
                            startX : 1.263158,
                            endX : 1.368421,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 1.368421,
                                    y : 1.5,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.473684,
                                    y : -1.0,
                                },
                            },
                            startX : 1.368421,
                            endX : 1.473684,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 1.473684,
                                    y : -1.0,
                                },
                                startWeight : 0.5,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.578947,
                                    y : 0.5,
                                },
                                endWeight : 0.0,
                            },
                            startX : 1.473684,
                            endX : (1.0/0.0),
                        },
                    },
                },
                holdLength : 17.68421,
                pointCount : 200,
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 11.36842,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.5,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 11.68421,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 11.89474,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
                },
                distance : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.LinearFunctionCurve : {
                                k : -12.0,
                                b : 0.0,
                            },
                            startX : (-1.0/0.0),
                            endX : 0.0,
                        },
                    },
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 12.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.5,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 12.10526,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 12.21053,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.5,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 12.31579,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 12.42105,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.5,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 12.52632,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 12.63158,
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
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 12.94737,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.0,
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
                holdTime : 0.4210526,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.1052632,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.2105263,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.4210526,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startWeight : 0.5,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.4210526,
                        y : 6.0,
                    },
                    endTangent : 0.0,
                    endWeight : 0.0,
                },
                holdLength : 5.052631,
            },
            Dremu.DremuHold : {
                laneName : "Main4",
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
                holdTime : 0.2105263,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.1052632,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.2105263,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startWeight : 0.5,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.2105263,
                        y : -4.0,
                    },
                    endTangent : 0.0,
                    endWeight : 0.0,
                },
                holdLength : 2.526316,
            },
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 13.89474,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
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
                holdTime : 1.157895,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.4210526,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.5263158,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.6315789,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.7368421,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.8421053,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.9473684,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.052632,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.157895,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.ConstantFunctionCurve : { : },
                            startX : 0.0,
                            endX : 0.3157895,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.3157895,
                                    y : 0.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.4210526,
                                    y : 3.0,
                                },
                            },
                            startX : 0.3157895,
                            endX : 0.4210526,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.4210526,
                                    y : 3.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.5263158,
                                    y : 1.5,
                                },
                            },
                            startX : 0.4210526,
                            endX : 0.5263158,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.5263158,
                                    y : 1.5,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.6315789,
                                    y : 4.5,
                                },
                            },
                            startX : 0.5263158,
                            endX : 0.6315789,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.6315789,
                                    y : 4.5,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.7368421,
                                    y : 3.0,
                                },
                            },
                            startX : 0.6315789,
                            endX : 0.7368421,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.7368421,
                                    y : 3.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.8421053,
                                    y : 6.0,
                                },
                            },
                            startX : 0.7368421,
                            endX : 0.8421053,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.8421053,
                                    y : 6.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.9473684,
                                    y : 4.5,
                                },
                            },
                            startX : 0.8421053,
                            endX : 0.9473684,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.9473684,
                                    y : 4.5,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.052632,
                                    y : 7.5,
                                },
                            },
                            startX : 0.9473684,
                            endX : 1.052632,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 1.052632,
                                    y : 7.5,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.157895,
                                    y : 6.0,
                                },
                            },
                            startX : 1.052632,
                            endX : (1.0/0.0),
                        },
                    },
                },
                holdLength : 13.89474,
            },
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 15.1579,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
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
                holdTime : 0.3157895,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startWeight : 0.5,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.3157895,
                        y : 7.0,
                    },
                    endTangent : 0.0,
                    endWeight : 0.0,
                },
                holdLength : 3.789474,
            },
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 15.47368,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
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
                holdTime : 0.8421053,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.4210526,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.5263158,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.6315789,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.7368421,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.8421053,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.0,
                                    y : 0.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.3157895,
                                    y : 7.0,
                                },
                            },
                            startX : 0.0,
                            endX : 0.3157895,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.3157895,
                                    y : 7.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.5263158,
                                    y : 3.0,
                                },
                            },
                            startX : 0.3157895,
                            endX : 0.5263158,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.5263158,
                                    y : 3.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.6315789,
                                    y : 4.0,
                                },
                            },
                            startX : 0.5263158,
                            endX : 0.6315789,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.6315789,
                                    y : 4.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.8421053,
                                    y : 0.0,
                                },
                            },
                            startX : 0.6315789,
                            endX : (1.0/0.0),
                        },
                    },
                },
                holdLength : 10.10526,
            },
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 16.42105,
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
                holdTime : 1.263158,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.263158,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startTangent : -16.886,
                    startWeight : 0.5,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 1.263158,
                        y : 0.0,
                    },
                    endTangent : -16.886,
                    endWeight : 0.5,
                },
                holdLength : 15.1579,
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 17.05263,
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
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 17.68421,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.0,
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
                holdTime : 1.263158,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.263158,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startTangent : 16.886,
                    startWeight : 0.5,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 1.263158,
                        y : 0.0,
                    },
                    endTangent : 16.886,
                    endWeight : 0.5,
                },
                holdLength : 15.1579,
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 18.31579,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.0,
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
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 18.94737,
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
                holdTime : 1.894737,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.894737,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -12.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.ConstantFunctionCurve : { : },
                holdLength : 22.73684,
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 21.47368,
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
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0,
                    g : 0.6588235,
                    b : 0.5882353,
                },
                respondHintColor2 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 24.31579,
                leadTime : 0.3157895,
                lagTime : 0.6,
                distance : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : -0.3157895,
                        y : 0.0,
                    },
                    startTangent : -60.0,
                    startWeight : 0.2,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    endTangent : -60.0,
                    endWeight : 0.2,
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
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 24.63158,
                leadTime : 0.3157895,
                lagTime : 0.6,
                distance : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : -0.3157895,
                        y : 0.0,
                    },
                    startTangent : 60.0,
                    startWeight : 0.2,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    endTangent : 60.0,
                    endWeight : 0.2,
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
                holdTime : 0.3157895,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.2105263,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                },
                endDistance : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -0.6315789,
                                    y : 0.0,
                                },
                                startTangent : 60.0,
                                startWeight : 0.2,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : -0.3157895,
                                    y : 0.0,
                                },
                                endTangent : 60.0,
                                endWeight : 0.2,
                            },
                            startX : (-1.0/0.0),
                            endX : -0.4736842,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.ConstantFunctionCurve : {
                                value : 2.8422,
                            },
                            startX : -0.4736842,
                            endX : -0.1578947,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : -0.3157895,
                                    y : 0.0,
                                },
                                startTangent : 60.0,
                                startWeight : 0.2,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.0,
                                    y : 0.0,
                                },
                                endTangent : 60.0,
                                endWeight : 0.2,
                            },
                            startX : -0.1578947,
                            endX : 0.0,
                        },
                    },
                },
                holdLength : 3.789474,
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 25.26316,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 11.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 25.57895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -11.0,
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
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 25.89474,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 10.0,
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
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 26.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 10.0,
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
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 26.10526,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 10.0,
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
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 26.21053,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
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
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 26.31579,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
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
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 26.42105,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.0,
                },
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
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 26.52632,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 9.0,
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
                holdTime : 1.263158,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.263158,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : 9.0,
                    b : 0.0,
                },
            },
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 27.15789,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -9.0,
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
                holdTime : 0.6315789,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.6315789,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -9.0,
                    b : 0.0,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 28.21053,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 28.31579,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 28.42105,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 28.52632,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 28.63158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 28.73684,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 28.84211,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 28.94737,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
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
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 29.05263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
                holdTime : 2.526316,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.5263158,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.6315789,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.9473684,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.263158,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.894737,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 2.526316,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.0,
                                    y : 0.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.6315789,
                                    y : 4.0,
                                },
                            },
                            startX : (-1.0/0.0),
                            endX : 0.6315789,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.6315789,
                                    y : 4.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.263158,
                                    y : -4.0,
                                },
                            },
                            startX : 0.6315789,
                            endX : 1.263158,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 1.263158,
                                    y : -4.0,
                                },
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.894737,
                                    y : 4.0,
                                },
                            },
                            startX : 1.263158,
                            endX : 1.894737,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 1.894737,
                                    y : 4.0,
                                },
                                startTangent : 0.0,
                                startWeight : 0.8,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 2.526316,
                                    y : -4.0,
                                },
                                endTangent : 0.0,
                                endWeight : 0.0,
                            },
                            startX : 1.894737,
                            endX : (1.0/0.0),
                        },
                    },
                },
                holdLength : 20.21053,
            },
            Dremu.DremuTap : {
                laneName : "Guide4B2",
                hitTime : 32.21053,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
            Dremu.DremuDrag : {
                laneName : "Guide4B2",
                hitTime : 32.42105,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
            Dremu.DremuDrag : {
                laneName : "Guide4B2",
                hitTime : 32.52632,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
            Dremu.DremuDrag : {
                laneName : "Guide4B2",
                hitTime : 32.63158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
            Dremu.DremuDrag : {
                laneName : "Guide4B2",
                hitTime : 32.73684,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
            Dremu.DremuTap : {
                laneName : "Guide4B4",
                hitTime : 33.47368,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
            Dremu.DremuTap : {
                laneName : "Guide4B1",
                hitTime : 33.47368,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
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
            Dremu.DremuTap : {
                laneName : "Guide4B4",
                hitTime : 33.78947,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
            Dremu.DremuTap : {
                laneName : "Guide4B1",
                hitTime : 33.78947,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
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
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 34.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.5,
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
                holdTime : 2.0,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.7368421,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.368421,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 2.0,
                    },
                },
                holdLine : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.0,
                                    y : 0.0,
                                },
                                startTangent : 0.0,
                                startWeight : 0.0,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.7368421,
                                    y : -1.5,
                                },
                                endTangent : 0.0,
                                endWeight : 0.5,
                            },
                            startX : 0.0,
                            endX : 0.7368421,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.7368421,
                                    y : -1.5,
                                },
                                startTangent : 0.0,
                                startWeight : 0.5,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.368421,
                                    y : 0.0,
                                },
                                endTangent : 3.0,
                                endWeight : 0.0,
                            },
                            startX : 0.7368421,
                            endX : 1.368421,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 1.368421,
                                    y : 0.0,
                                },
                                startTangent : 0.0,
                                startWeight : 0.0,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 2.0,
                                    y : 1.5,
                                },
                                endTangent : 0.0,
                                endWeight : 0.5,
                            },
                            startX : 1.368421,
                            endX : (1.0/0.0),
                        },
                    },
                },
                holdLength : 16.0,
            },
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 34.10526,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
                holdTime : 1.894737,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.6315789,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.263158,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.894737,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.PiecewiseFunctionCurve : {
                    functionPieces : GorgeFramework.FunctionPiece^ : {
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.0,
                                    y : 0.0,
                                },
                                startTangent : 0.0,
                                startWeight : 0.0,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 0.6315789,
                                    y : 1.5,
                                },
                                endTangent : 0.0,
                                endWeight : 0.5,
                            },
                            startX : 0.0,
                            endX : 0.6315789,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 0.6315789,
                                    y : 1.5,
                                },
                                startTangent : 0.0,
                                startWeight : 0.5,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.263158,
                                    y : 0.0,
                                },
                                endTangent : 3.0,
                                endWeight : 0.0,
                            },
                            startX : 0.6315789,
                            endX : 1.263158,
                        },
                        GorgeFramework.FunctionPiece : {
                            functionCurve : GorgeFramework.CubicHermiteSpline : {
                                startPoint : GorgeFramework.Vector2 : {
                                    x : 1.263158,
                                    y : 0.0,
                                },
                                startTangent : 0.0,
                                startWeight : 0.0,
                                endPoint : GorgeFramework.Vector2 : {
                                    x : 1.894737,
                                    y : -1.5,
                                },
                                endTangent : 0.0,
                                endWeight : 0.5,
                            },
                            startX : 1.263158,
                            endX : (1.0/0.0),
                        },
                    },
                },
                holdLength : 15.1579,
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 36.63158,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 37.57895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 37.89474,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 38.84211,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
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
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 39.05263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
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
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 39.78947,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
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
                holdTime : 1.894737,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.263158,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.368421,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.263158,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.578947,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.684211,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.789474,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.894737,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 1.263158,
                        y : 0.0,
                    },
                    startTangent : 0.0,
                    startWeight : 0.8,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 1.894737,
                        y : -8.0,
                    },
                    endTangent : 0.0,
                    endWeight : 0.0,
                },
                holdLength : -15.1579,
            },
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 40.42105,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.0,
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
                holdTime : 1.263158,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.6315789,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.7368421,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.8421053,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.9473684,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.052632,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 1.263158,
                    },
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.6315789,
                        y : 0.0,
                    },
                    startTangent : 0.0,
                    startWeight : 0.8,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 1.263158,
                        y : 8.0,
                    },
                    endTangent : 0.0,
                    endWeight : 0.0,
                },
                holdLength : 10.10526,
            },
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 42.31579,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
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
                holdTime : 0.6315789,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.1052632,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.2105263,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.4210526,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.5263158,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.6315789,
                    },
                },
                endDistance : GorgeFramework.LinearFunctionCurve : {
                    k : 8.0,
                    b : 0.0,
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startTangent : 0.0,
                    startWeight : 0.8,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.6315789,
                        y : -8.0,
                    },
                    endTangent : 0.0,
                    endWeight : 0.0,
                },
                holdLength : -5.052631,
            },
            Dremu.DremuHold : {
                laneName : "Main4",
                hitTime : 42.31579,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.0,
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
                holdTime : 0.6315789,
                innerNotes : Dremu.DremuHoldInnerNote^ : {
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.1052632,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.2105263,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.3157895,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.4210526,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.5263158,
                    },
                    Dremu.DremuHoldInnerNote : {
                        hitTime : 0.6315789,
                    },
                },
                holdLine : GorgeFramework.CubicHermiteSpline : {
                    startPoint : GorgeFramework.Vector2 : {
                        x : 0.0,
                        y : 0.0,
                    },
                    startTangent : 0.0,
                    startWeight : 0.8,
                    endPoint : GorgeFramework.Vector2 : {
                        x : 0.6315789,
                        y : 8.0,
                    },
                    endTangent : 0.0,
                    endWeight : 0.0,
                },
                holdLength : 5.052631,
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 43.57895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                },
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 44.21053,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -6.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 10.0,
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 44.52632,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -9.0,
                },
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 44.84211,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -12.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 10.0,
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuTap : {
                laneName : "Main4",
                hitTime : 45.15789,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -15.0,
                },
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 45.31579,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -16.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 10.0,
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 45.47368,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -18.0,
                },
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 45.57895,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -19.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 10.0,
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 45.68421,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -20.0,
                },
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 45.78947,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -21.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 10.0,
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 45.89474,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -22.0,
                },
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 46.0,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -23.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 10.0,
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 46.10526,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -24.0,
                },
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 46.21053,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -25.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 10.0,
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 46.31579,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -26.0,
                },
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
            Dremu.DremuDrag : {
                laneName : "Main4",
                hitTime : 46.42105,
                lagTime : 0.6,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -27.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : 10.0,
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
                    r : 0.9411765,
                    g : 0.9490196,
                    b : 0.7568628,
                },
            },
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 107.1053,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period3()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -5.0,
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
                hitTime : 0.2105263,
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
                hitTime : 0.3157895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.0,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 126.0526,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period4()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                leadTime : 3.0,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -4.5,
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
                hitTime : 0.2105263,
                leadTime : 3.0,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -4.5,
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
                hitTime : 0.3157895,
                leadTime : 3.0,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -4.5,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                leadTime : 3.0,
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -4.5,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 124.7895,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period5()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -5.0,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.2105263,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -5.0,
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
                hitTime : 0.3157895,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -0.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -5.0,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -0.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -5.0,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 123.5263,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period6()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -5.5,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.2105263,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -5.5,
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
                hitTime : 0.3157895,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -5.5,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -5.5,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 122.2632,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period7()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -6.0,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.2105263,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -6.0,
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
                hitTime : 0.3157895,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -6.0,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -6.0,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 121.0,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period8()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -6.5,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.2105263,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -6.5,
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
                hitTime : 0.3157895,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -6.5,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -6.5,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 119.7368,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period9()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -7.0,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.2105263,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -7.0,
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
                hitTime : 0.3157895,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -7.0,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -7.0,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 118.4737,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period10()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -7.5,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.2105263,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -7.5,
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
                hitTime : 0.3157895,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -7.5,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                leadTime : 3.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -7.5,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 117.2105,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period11()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.2105263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
                hitTime : 0.3157895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.0,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 115.9474,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period12()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.5,
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
                hitTime : 0.2105263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.5,
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
                hitTime : 0.3157895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.5,
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
                hitTime : 0.5263158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.5,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 114.6842,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period13()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -9.0,
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
                hitTime : 0.2105263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -0.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -9.0,
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
                hitTime : 0.3157895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -9.0,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -9.0,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 113.4211,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period14()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -9.5,
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
                hitTime : 0.2105263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -9.5,
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
                hitTime : 0.3157895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -9.5,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -9.5,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 112.1579,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period15()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.5,
                },
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
                hitTime : 0.2105263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.5,
                },
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
                hitTime : 0.3157895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
                },
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
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
                    a : 1.0,
                    r : 0.0196078,
                    g : 0.7607843,
                    b : 0.5882353,
                },
            },
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.5,
                },
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
                    progressCurve : null,
                },
                respondHintColor1 : GorgeFramework.ColorArgb : {
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
            timeOffset : 110.8947,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period16()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.5,
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
                hitTime : 0.2105263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.5,
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
                hitTime : 0.3157895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.5,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -10.5,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 109.6316,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period17()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -11.0,
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
                hitTime : 0.2105263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -11.0,
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
                hitTime : 0.3157895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -11.0,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.5,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -11.0,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 108.3684,
            minLength : 0.9473684,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period18()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -4.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -11.5,
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
                hitTime : 0.2105263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -11.5,
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
                hitTime : 0.3157895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -3.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -11.5,
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
            Dremu.DremuTaplik : {
                laneName : "Main6",
                hitTime : 0.5263158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 4.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -11.5,
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
        };
    }


    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 116.5789,
            minLength : 0.6315789,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period19()
    {
        return new GorgeFramework.Element^[4]{
            Dremu.DremuTap : {
                laneName : "Main6",
                hitTime : 0.0,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.25,
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
                hitTime : 0.2105263,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -2.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.25,
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
                hitTime : 0.3157895,
                position : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.25,
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
                hitTime : 0.5263158,
                position : GorgeFramework.VariableFloat : {
                    baseValue : -1.0,
                },
                distance : GorgeFramework.LinearFunctionCurve : {
                    k : -8.25,
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
        };
    }


}
