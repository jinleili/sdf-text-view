#ifdef GL_FRAGMENT_PRECISION_HIGH
    precision highp float;
#else
    precision mediump float;
#endif

#ifdef ES_20
    #define in varying
    #define frag_color gl_FragColor
    #define texture texture2D
#endif

#ifdef ES_30
    out vec4 frag_color;
#endif

#ifdef DESKTOP
    out vec4 frag_color;
#endif