[
    string form = "Dremu",
    string displayName = "表演"
]
@ElementStaff
class DremuStaff
{
    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : 1.0,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period()
    {
        return new GorgeFramework.Element^[11]{
            Dremu.DremuMainLane : {
                name : "ArtLine1",
                generateTime : 0.0,
                keepTime : 1.052632,
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : -80.0,
                        startWeight : 0.035,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.6,
                        },
                        endTangent : 0.0,
                        endWeight : 0.0,
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : 0.0,
                        startWeight : 0.0,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.6,
                        },
                        endTangent : -80.0,
                        endWeight : 0.035,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
            },
            Dremu.DremuMainLane : {
                name : "ArtLine2",
                generateTime : 1.263158,
                keepTime : 1.052632,
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : -80.0,
                        startWeight : 0.035,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.6,
                        },
                        endTangent : 0.0,
                        endWeight : 0.0,
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : 0.0,
                        startWeight : 0.0,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.6,
                        },
                        endTangent : -80.0,
                        endWeight : 0.035,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
            },
            Dremu.DremuMainLane : {
                name : "ArtLine3",
                generateTime : 2.526316,
                keepTime : 1.052632,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CompositeFunctionCurve : {
                                    outerFunctionCurve : GorgeFramework.LinearFunctionCurve : {
                                        k : -0.15,
                                    },
                                    innerFunctionCurve : GorgeFramework.CompositeFunctionCurve : {
                                        outerFunctionCurve : GorgeFramework.PeriodicFunctionCurve : {
                                            functionCurve : GorgeFramework.AxialSymmetricFunctionCurve : {
                                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                                    k : 1.0,
                                                },
                                                axis : 1.0,
                                            },
                                            startX : -1.0,
                                            endX : 3.0,
                                        },
                                        innerFunctionCurve : GorgeFramework.LinearFunctionCurve : {
                                            k : 6.666667,
                                        },
                                    },
                                },
                                startX : -0.3,
                                endX : 0.3,
                            },
                        },
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : -80.0,
                        startWeight : 0.035,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.6,
                        },
                        endTangent : 0.0,
                        endWeight : 0.0,
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : 0.0,
                        startWeight : 0.0,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.6,
                        },
                        endTangent : -80.0,
                        endWeight : 0.035,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
            },
            Dremu.DremuMainLane : {
                name : "ArtLine4",
                generateTime : 3.789474,
                keepTime : 1.052632,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CompositeFunctionCurve : {
                                    outerFunctionCurve : GorgeFramework.LinearFunctionCurve : {
                                        k : 0.2,
                                        b : 0.0,
                                    },
                                    innerFunctionCurve : GorgeFramework.CompositeFunctionCurve : {
                                        outerFunctionCurve : GorgeFramework.PeriodicFunctionCurve : {
                                            functionCurve : GorgeFramework.AxialSymmetricFunctionCurve : {
                                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                                    k : 1.0,
                                                },
                                                axis : 1.0,
                                            },
                                            startX : -1.0,
                                            endX : 3.0,
                                        },
                                        innerFunctionCurve : GorgeFramework.LinearFunctionCurve : {
                                            k : 5.0,
                                            b : 1.0,
                                        },
                                    },
                                },
                                startX : -1.0,
                                endX : 1.0,
                            },
                        },
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : -80.0,
                        startWeight : 0.035,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.6,
                        },
                        endTangent : 0.0,
                        endWeight : 0.0,
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : 0.0,
                        startWeight : 0.0,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.6,
                        },
                        endTangent : -80.0,
                        endWeight : 0.035,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
            },
            Dremu.DremuMainLane : {
                name : "ArtLine7A",
                generateTime : 7.578948,
                keepTime : 1.263158,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CompositeFunctionCurve : {
                                    outerFunctionCurve : GorgeFramework.LinearFunctionCurve : {
                                        k : 0.2,
                                        b : 0.0,
                                    },
                                    innerFunctionCurve : GorgeFramework.CompositeFunctionCurve : {
                                        outerFunctionCurve : GorgeFramework.PeriodicFunctionCurve : {
                                            functionCurve : GorgeFramework.AxialSymmetricFunctionCurve : {
                                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                                    k : 1.0,
                                                },
                                                axis : 1.0,
                                            },
                                            startX : -1.0,
                                            endX : 3.0,
                                        },
                                        innerFunctionCurve : GorgeFramework.LinearFunctionCurve : {
                                            k : 5.0,
                                            b : 1.0,
                                        },
                                    },
                                },
                                startX : -1.0,
                                endX : 1.0,
                            },
                        },
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : -70.0,
                        startWeight : 0.035,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.0,
                        },
                        endTangent : 0.0,
                        endWeight : 0.3,
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.0,
                                        y : 1.6,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.3,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 0.852632,
                                        y : 1.0,
                                    },
                                    endTangent : -3.0,
                                    endWeight : 0.035,
                                },
                                endX : 1.052632,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.052632,
                                        y : 1.0,
                                    },
                                    startTangent : -60.0,
                                    startWeight : 0.15,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 1.263158,
                                        y : -1.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.0,
                                },
                                endX : 1.263158,
                            },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
            },
            Dremu.DremuMainLane : {
                name : "ArtLine7B",
                generateTime : 7.578948,
                keepTime : 1.263158,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CompositeFunctionCurve : {
                                    outerFunctionCurve : GorgeFramework.LinearFunctionCurve : {
                                        k : 0.2,
                                        b : 0.0,
                                    },
                                    innerFunctionCurve : GorgeFramework.CompositeFunctionCurve : {
                                        outerFunctionCurve : GorgeFramework.PeriodicFunctionCurve : {
                                            functionCurve : GorgeFramework.AxialSymmetricFunctionCurve : {
                                                functionCurve : GorgeFramework.LinearFunctionCurve : {
                                                    k : 1.0,
                                                },
                                                axis : 1.0,
                                            },
                                            startX : -1.0,
                                            endX : 3.0,
                                        },
                                        innerFunctionCurve : GorgeFramework.LinearFunctionCurve : {
                                            k : 5.0,
                                            b : 1.0,
                                        },
                                    },
                                },
                                startX : -1.0,
                                endX : 1.0,
                            },
                        },
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : -70.0,
                        startWeight : 0.035,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.0,
                        },
                        endTangent : 0.0,
                        endWeight : 0.3,
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.0,
                                        y : 1.6,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.3,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 0.852632,
                                        y : 1.0,
                                    },
                                    endTangent : -3.0,
                                    endWeight : 0.035,
                                },
                                endX : 1.052632,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 1.052632,
                                        y : 1.0,
                                    },
                                    startTangent : -60.0,
                                    startWeight : 0.15,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 1.263158,
                                        y : -1.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.0,
                                },
                                endX : 1.263158,
                            },
                        },
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
                positionY : GorgeFramework.VariableFloat : {
                    baseValue : -0.8,
                },
                rotationZ : GorgeFramework.VariableFloat : {
                    baseValue : 180.0,
                },
            },
            Dremu.DremuMainLane : {
                name : "ArtLine5",
                generateTime : 5.052631,
                keepTime : 1.052632,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : -1.0,
                                    valueStart : 0.0,
                                    timeEnd : -0.8,
                                    valueEnd : -0.1,
                                },
                                startX : -1.0,
                                endX : -0.8,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : -0.8,
                                    valueStart : -0.1,
                                    timeEnd : -0.4,
                                    valueEnd : 0.2,
                                },
                                startX : -0.8,
                                endX : -0.4,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : -0.4,
                                    valueStart : 0.2,
                                    timeEnd : 0.0,
                                    valueEnd : -0.3,
                                },
                                startX : -0.4,
                                endX : 0.0,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : 0.0,
                                    valueStart : -0.3,
                                    timeEnd : 0.4,
                                    valueEnd : 0.1,
                                },
                                startX : 0.0,
                                endX : 0.4,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : 0.4,
                                    valueStart : 0.1,
                                    timeEnd : 0.8,
                                    valueEnd : -0.1,
                                },
                                startX : 0.4,
                                endX : 0.8,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : 0.8,
                                    valueStart : -0.1,
                                    timeEnd : 1.0,
                                    valueEnd : 0.0,
                                },
                                startX : 0.8,
                                endX : 1.0,
                            },
                        },
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : -80.0,
                        startWeight : 0.035,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.6,
                        },
                        endTangent : 0.0,
                        endWeight : 0.0,
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : 0.0,
                        startWeight : 0.0,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.6,
                        },
                        endTangent : -80.0,
                        endWeight : 0.035,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
            },
            Dremu.DremuMainLane : {
                name : "ArtLine6",
                generateTime : 6.31579,
                keepTime : 1.052632,
                laneLines : GorgeFramework.FunctionCurve^ : {
                    GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : -1.0,
                                    valueStart : 0.0,
                                    timeEnd : -0.8,
                                    valueEnd : 0.1,
                                },
                                startX : -1.0,
                                endX : -0.8,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : -0.8,
                                    valueStart : 0.1,
                                    timeEnd : -0.4,
                                    valueEnd : -0.2,
                                },
                                startX : -0.8,
                                endX : -0.4,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : -0.4,
                                    valueStart : -0.2,
                                    timeEnd : 0.0,
                                    valueEnd : 0.3,
                                },
                                startX : -0.4,
                                endX : 0.0,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : 0.0,
                                    valueStart : 0.3,
                                    timeEnd : 0.4,
                                    valueEnd : -0.25,
                                },
                                startX : 0.0,
                                endX : 0.4,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : 0.4,
                                    valueStart : -0.25,
                                    timeEnd : 0.8,
                                    valueEnd : 0.15,
                                },
                                startX : 0.4,
                                endX : 0.8,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.LinearCurve : {
                                    timeStart : 0.8,
                                    valueStart : 0.15,
                                    timeEnd : 1.0,
                                    valueEnd : 0.0,
                                },
                                startX : 0.8,
                                endX : 1.0,
                            },
                        },
                    },
                },
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : -80.0,
                        startWeight : 0.035,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.6,
                        },
                        endTangent : 0.0,
                        endWeight : 0.0,
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : 1.6,
                        },
                        startTangent : 0.0,
                        startWeight : 0.0,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 1.052632,
                            y : -1.6,
                        },
                        endTangent : -80.0,
                        endWeight : 0.035,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
            },
            Dremu.DremuMainLane : {
                name : "ArtLine4.5",
                generateTime : 4.842105,
                keepTime : 0.2105263,
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : -1.6,
                        },
                        startTangent : 100.0,
                        startWeight : 0.15,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 0.2105263,
                            y : 1.6,
                        },
                        endTangent : 0.0,
                        endWeight : 0.5,
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : -1.6,
                        },
                        startTangent : 0.0,
                        startWeight : 0.5,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 0.2105263,
                            y : 1.6,
                        },
                        endTangent : 100.0,
                        endWeight : 0.15,
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
                    progressCurve : null,
                },
            },
            Dremu.DremuMainLane : {
                name : "ArtLine5.5",
                generateTime : 6.105263,
                keepTime : 0.2105263,
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : -1.6,
                        },
                        startTangent : 100.0,
                        startWeight : 0.15,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 0.2105263,
                            y : 1.6,
                        },
                        endTangent : 0.0,
                        endWeight : 0.5,
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : -1.6,
                        },
                        startTangent : 0.0,
                        startWeight : 0.5,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 0.2105263,
                            y : 1.6,
                        },
                        endTangent : 100.0,
                        endWeight : 0.15,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
            },
            Dremu.DremuMainLane : {
                name : "ArtLine6.5",
                generateTime : 7.368421,
                keepTime : 0.2105263,
                drawStartX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : -1.6,
                        },
                        startTangent : 100.0,
                        startWeight : 0.15,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 0.2105263,
                            y : 1.6,
                        },
                        endTangent : 0.0,
                        endWeight : 0.5,
                    },
                },
                drawEndX : GorgeFramework.VariableFloat : {
                    baseValue : 0.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : -1.6,
                        },
                        startTangent : 0.0,
                        startWeight : 0.5,
                        endPoint : GorgeFramework.Vector2 : {
                            x : 0.2105263,
                            y : 1.6,
                        },
                        endTangent : 100.0,
                        endWeight : 0.15,
                    },
                },
                color : GorgeFramework.LerpColorCurve : {
                    colorPoints : GorgeFramework.ColorArgb^ : {
                        GorgeFramework.ColorArgb : {
                            r : 0.0196078,
                            g : 0.7607843,
                            b : 0.5882353,
                        },
                    },
                    progressCurve : null,
                },
            },
        };
    }


}
