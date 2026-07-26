[
    string form = "DeentyStoryboard",
    string displayName = "Storyboard"
]
@ElementStaff
class DeentyStoryboardStaff
{
    [
        GorgeFramework.PeriodConfig^ config = GorgeFramework.PeriodConfig : {
            timeOffset : -0.2631579,
        }
    ]
    @Chart
    static GorgeFramework.Element^[] Period()
    {
        return new GorgeFramework.Element^[8]{
            DeentyStoryboard.Image : {
                assetName : "image:Background1",
                keepTime : 22.73684,
                positionMode : DeentyStoryboard.ImagePositionMode.ScreenCover,
                positionZ : GorgeFramework.VariableFloat : {
                    baseValue : -0.9,
                },
                alpha : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : -1.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 0.0277778,
                            y : 0.0,
                        },
                    },
                },
            },
            DeentyStoryboard.Image : {
                assetName : "image:Background2",
                startMoment : 21.47368,
                keepTime : 21.47368,
                positionMode : DeentyStoryboard.ImagePositionMode.ScreenCover,
                positionZ : GorgeFramework.VariableFloat : {
                    baseValue : -0.901,
                },
                alpha : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                    variationCurve : GorgeFramework.LinearCurve : {
                        timeStart : 0.0,
                        valueStart : -1.0,
                        timeEnd : 0.0073529,
                        valueEnd : 0.0,
                    },
                },
            },
            DeentyStoryboard.Image : {
                assetName : "image:Block",
                startMoment : 41.05263,
                keepTime : 1.263158,
                positionMode : DeentyStoryboard.ImagePositionMode.ScreenY,
                scaleY : GorgeFramework.VariableFloat : {
                    baseValue : 0.8,
                    variationCurve : GorgeFramework.PiecewiseFunctionCurve : {
                        functionPieces : GorgeFramework.FunctionPiece^ : {
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.0,
                                        y : -0.8,
                                    },
                                    startTangent : 12.0,
                                    startWeight : 0.8,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 0.125,
                                        y : 0.0,
                                    },
                                    endTangent : -2.0,
                                    endWeight : 0.1,
                                },
                                startX : 0.0,
                                endX : 0.125,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.125,
                                        y : 0.0,
                                    },
                                    startTangent : -2.0,
                                    startWeight : 0.1,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 0.5,
                                        y : 0.0,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.5,
                                },
                                startX : 0.125,
                                endX : 0.5,
                            },
                            GorgeFramework.FunctionPiece : {
                                functionCurve : GorgeFramework.CubicHermiteSpline : {
                                    startPoint : GorgeFramework.Vector2 : {
                                        x : 0.5,
                                        y : 0.0,
                                    },
                                    startTangent : 0.0,
                                    startWeight : 0.5,
                                    endPoint : GorgeFramework.Vector2 : {
                                        x : 0.625,
                                        y : 2.5,
                                    },
                                    endTangent : 0.0,
                                    endWeight : 0.0,
                                },
                                startX : 0.5,
                                endX : 1.0,
                            },
                        },
                    },
                },
                positionZ : GorgeFramework.VariableFloat : {
                    baseValue : -0.902,
                },
            },
            DeentyStoryboard.Image : {
                assetName : "image:Block",
                startMoment : 42.0,
                keepTime : 5.052631,
                positionMode : DeentyStoryboard.ImagePositionMode.ScreenY,
                scaleY : GorgeFramework.VariableFloat : {
                    baseValue : 2.5,
                },
                positionZ : GorgeFramework.VariableFloat : {
                    baseValue : -0.902,
                },
            },
            DeentyStoryboard.Image : {
                assetName : "image:Background3",
                startMoment : 42.94737,
                keepTime : 45.47368,
                positionMode : DeentyStoryboard.ImagePositionMode.ScreenCover,
                positionZ : GorgeFramework.VariableFloat : {
                    baseValue : -0.903,
                },
                alpha : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                    variationCurve : GorgeFramework.LinearCurve : {
                        timeStart : 0.0,
                        valueStart : -1.0,
                        timeEnd : 0.0833333,
                    },
                },
            },
            DeentyStoryboard.Image : {
                assetName : "image:Background1",
                startMoment : 61.89474,
                keepTime : 2.526316,
                positionMode : DeentyStoryboard.ImagePositionMode.ScreenCover,
                positionZ : GorgeFramework.VariableFloat : {
                    baseValue : -0.904,
                },
            },
            DeentyStoryboard.Image : {
                assetName : "image:Background4",
                startMoment : 84.63158,
                keepTime : 25.26316,
                positionMode : DeentyStoryboard.ImagePositionMode.ScreenCover,
                positionZ : GorgeFramework.VariableFloat : {
                    baseValue : -0.905,
                },
                alpha : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                    variationCurve : GorgeFramework.LinearCurve : {
                        timeStart : 0.0,
                        valueStart : -1.0,
                        timeEnd : 0.0875,
                        valueEnd : 0.0,
                    },
                },
            },
            DeentyStoryboard.Image : {
                assetName : "image:Background1",
                startMoment : 106.1053,
                keepTime : 31.57895,
                positionMode : DeentyStoryboard.ImagePositionMode.ScreenCover,
                positionZ : GorgeFramework.VariableFloat : {
                    baseValue : -0.906,
                },
                alpha : GorgeFramework.VariableFloat : {
                    baseValue : 1.0,
                    variationCurve : GorgeFramework.CubicHermiteSpline : {
                        startPoint : GorgeFramework.Vector2 : {
                            x : 0.0,
                            y : -1.0,
                        },
                        endPoint : GorgeFramework.Vector2 : {
                            x : 0.04,
                            y : 0.0,
                        },
                    },
                },
            },
        };
    }


}
