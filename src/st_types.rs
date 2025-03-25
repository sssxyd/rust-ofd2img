use std::{num::ParseFloatError, str::FromStr};

#[derive(Debug)]
pub(crate) enum ParseSTError {
    InvalidFormat,
    ParseFloatError(ParseFloatError),
}

#[derive(Debug, Default, Clone)]
pub(crate) struct STPos {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl FromStr for STPos {
    type Err = ParseSTError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts : Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(ParseSTError::InvalidFormat);
        }
        // 解析 x 和 y
        let x = parts[0]
            .parse::<f64>()
            .map_err(ParseSTError::ParseFloatError)?;
        let y = parts[1]
            .parse::<f64>()
            .map_err(ParseSTError::ParseFloatError)?;

        Ok(STPos { x, y })
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct STBox {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) w: f64,
    pub(crate) h: f64,
}

impl FromStr for STBox {
    type Err = ParseSTError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts : Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 4 {
            return Err(ParseSTError::InvalidFormat);
        }
        // 解析 x, y, w, h
        let x = parts[0]
            .parse::<f64>()
            .map_err(ParseSTError::ParseFloatError)?;
        let y = parts[1]
            .parse::<f64>()
            .map_err(ParseSTError::ParseFloatError)?;
        let w = parts[2]
            .parse::<f64>()
            .map_err(ParseSTError::ParseFloatError)?;
        let h = parts[3]
            .parse::<f64>()
            .map_err(ParseSTError::ParseFloatError)?;

        Ok(STBox { x, y, w, h })
    }
}

#[derive(Debug, Clone)]
pub(crate) enum PathElement {
    StartAt(StartAt),
    MoveTo(MoveTo),
    LineTo(LineTo),
    QuadraticBezierCurve(QuadraticBezierCurve),
    CubicBezierCurve(CubicBezierCurve),
    EllipseArc(EllipseArc),
    ClosePath(ClosePath),
}

/// 操作符 S 操作数 x y
/// 设置(x,y)为当前点
/// S x y
#[derive(Debug, Clone)]
pub(crate) struct StartAt {
    pub(crate) pos: STPos,
}

/// 操作符 M 操作数 x y
/// 从当前点到指定点移动，完成后设置(x,y)为当前点
/// M x y
#[derive(Debug, Clone)]
pub(crate) struct MoveTo {
    pub(crate) pos: STPos,
}

/// 操作符 L 操作数 x y
/// 从当前点到指定点画直线，完成后设置(x,y)为当前点
/// L x y
#[derive(Debug, Clone)]
pub(crate) struct LineTo {
    pub(crate) pos: STPos,
}

/// 操作符 Q 操作数 x1 y1 x y
/// 从当前点到指定点(x2,y2)画二次贝塞尔曲线，以(x1,y1)为控制点，完成后设置(x2,y2)为当前点
/// Q x1 y1 x2 y2
#[derive(Debug, Clone)]
pub(crate) struct QuadraticBezierCurve {
    pub(crate) pos1: STPos,
    pub(crate) pos2: STPos,
}

/// 操作符 B 操作数 x1 y1 x2 y2 x3 y3
/// 从当前点到指定点(x3,y3)画三次贝塞尔曲线，以(x1,y1)和(x2,y2)为控制点，完成后设置(x3,y3)为当前点
/// B x1 y1 x2 y2 x3 y3
#[derive(Debug, Clone)]
pub(crate) struct CubicBezierCurve {
    pub(crate) pos1: STPos,
    pub(crate) pos2: STPos,
    pub(crate) pos3: STPos,
}

/// 操作符 A 操作数 rx ry angle large sweep x y
/// 从当前点连接一条到点(x,y)的圆弧，并将当前点移动到点(x,y)。
/// rx表示椭圆的长轴长度，ry表示椭圆的短轴长度。
/// anglc表示椭圆在当前坐标系下旋转的角度，正值为顺时针，负值为逆时针，
/// largc为1时表示对应度数大于180°的弧，为0时表示对应度数小于180°的弧。
/// swecp为1时表示由圆弧起始点到结束点是顺时针旋转，为0时表示由圆弧起始点到结束点是逆时针旋转
/// A rx ry angle large sweep x y
#[derive(Debug, Clone)]
pub(crate) struct EllipseArc {
    pub(crate) rx: f64,
    pub(crate) ry: f64,
    pub(crate) angle: f64,
    pub(crate) large: f64,
    pub(crate) sweep: f64,
    pub(crate) pos: STPos,
}

/// 操作符 C 操作数 无
/// SubPath自动闭合，表示将当前点和SubPath的起始点用线段直接连接
/// C
#[derive(Debug, Clone)]
pub(crate) struct ClosePath {}


#[derive(Debug, Clone)]
pub(crate) struct STPath {
    pub(crate) elements: Vec<PathElement>,
}

impl FromStr for STPath {
    type Err = ParseSTError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = Vec::new();
        let parts: Vec<&str> = s.split_whitespace().collect();
        let mut tokens = parts.into_iter();

        while let Some(op) = tokens.next() {
            match op {
                "S" => {
                    let x_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let y_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let pos = STPos {
                        x: x_str.parse().map_err(ParseSTError::ParseFloatError)?,
                        y: y_str.parse().map_err(ParseSTError::ParseFloatError)?,
                    };
                    elements.push(PathElement::StartAt(StartAt { pos }));
                }
                "M" => {
                    let x_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let y_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let pos = STPos {
                        x: x_str.parse().map_err(ParseSTError::ParseFloatError)?,
                        y: y_str.parse().map_err(ParseSTError::ParseFloatError)?,
                    };
                    elements.push(PathElement::MoveTo(MoveTo { pos }));
                }
                "L" => {
                    let x_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let y_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let pos = STPos {
                        x: x_str.parse().map_err(ParseSTError::ParseFloatError)?,
                        y: y_str.parse().map_err(ParseSTError::ParseFloatError)?,
                    };
                    elements.push(PathElement::LineTo(LineTo { pos }));
                }
                "Q" => {
                    let x1_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let y1_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let x2_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let y2_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let pos1 = STPos {
                        x: x1_str.parse().map_err(ParseSTError::ParseFloatError)?,
                        y: y1_str.parse().map_err(ParseSTError::ParseFloatError)?,
                    };
                    let pos2 = STPos {
                        x: x2_str.parse().map_err(ParseSTError::ParseFloatError)?,
                        y: y2_str.parse().map_err(ParseSTError::ParseFloatError)?,
                    };
                    elements.push(PathElement::QuadraticBezierCurve(QuadraticBezierCurve {
                        pos1,
                        pos2,
                    }));
                }
                "B" => {
                    let x1_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let y1_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let x2_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let y2_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let x3_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let y3_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let pos1 = STPos {
                        x: x1_str.parse().map_err(ParseSTError::ParseFloatError)?,
                        y: y1_str.parse().map_err(ParseSTError::ParseFloatError)?,
                    };
                    let pos2 = STPos {
                        x: x2_str.parse().map_err(ParseSTError::ParseFloatError)?,
                        y: y2_str.parse().map_err(ParseSTError::ParseFloatError)?,
                    };
                    let pos3 = STPos {
                        x: x3_str.parse().map_err(ParseSTError::ParseFloatError)?,
                        y: y3_str.parse().map_err(ParseSTError::ParseFloatError)?,
                    };
                    elements.push(PathElement::CubicBezierCurve(CubicBezierCurve {
                        pos1,
                        pos2,
                        pos3,
                    }));
                }
                "A" => {
                    let rx_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let ry_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let angle_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let large_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let sweep_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let x_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let y_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let rx = rx_str.parse().map_err(ParseSTError::ParseFloatError)?;
                    let ry = ry_str.parse().map_err(ParseSTError::ParseFloatError)?;
                    let angle = angle_str.parse().map_err(ParseSTError::ParseFloatError)?;
                    let large = large_str.parse().map_err(ParseSTError::ParseFloatError)?;
                    let sweep = sweep_str.parse().map_err(ParseSTError::ParseFloatError)?;
                    let pos = STPos {
                        x: x_str.parse().map_err(ParseSTError::ParseFloatError)?,
                        y: y_str.parse().map_err(ParseSTError::ParseFloatError)?,
                    };
                    elements.push(PathElement::EllipseArc(EllipseArc {
                        rx,
                        ry,
                        angle,
                        large,
                        sweep,
                        pos,
                    }));
                }
                "C" => {
                    elements.push(PathElement::ClosePath(ClosePath {}));
                }
                _ => return Err(ParseSTError::InvalidFormat),
            }
        }

        Ok(STPath { elements })
    }
}

/// 每个字符相对于前一个字符的偏移量
/// 自动展开 g 语法的写法
#[derive(Debug, Clone)]
pub(crate) struct STDeltas {
    pub(crate) deltas: Vec<f64>,
}

impl FromStr for STDeltas {
    type Err = ParseSTError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts : Vec<&str> = s.split_whitespace().collect();
        let mut tokens = parts.into_iter();
        let mut deltas : Vec<f64> = Vec::new();
        while let Some(item) = tokens.next() {
            match item {
                "g" => {
                    let count_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let count: f64 = count_str.parse().map_err(ParseSTError::ParseFloatError)?;
                    let delta_str = tokens.next().ok_or(ParseSTError::InvalidFormat)?;
                    let delta = delta_str.parse().map_err(ParseSTError::ParseFloatError)?;
                    for _ in 0..count as i32 {
                        deltas.push(delta);
                    }
                }
                _ => {
                    let delta = item.parse().map_err(ParseSTError::ParseFloatError)?;
                    deltas.push(delta);
                }
            }
        }
        Ok(STDeltas { deltas })
    }
    
}