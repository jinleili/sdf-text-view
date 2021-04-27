// 加上这个精度在 ES 2.0 会有问题：报 uniform 常量的精度不对
// #ifdef GL_VERTEX_PRECISION_HIGH
//     precision highp float;
// #else
//     precision mediump float;
// #endif

#ifdef ES_20
    #define in attribute
    #define out varying
    #define texture texture2D
#endif 