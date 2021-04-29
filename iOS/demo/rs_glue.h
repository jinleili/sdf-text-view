//
//  rs_glue.h
//  vulkan_ios
//
//  Created by grenlight on 2018/11/23.
//

#ifndef rs_glue_h
#define rs_glue_h

struct idroid_obj;

struct app_view {
    void *view;
    // 获取 CAMetalLayer
    void *metal_layer;
    int maximum_frames;
    const char *temporary_directory;
    //   z float (*screen_scale)(void);
    void (*callback_to_swift)(int32_t arg);
};

struct view_size {
    int width;
    int height;
};

struct TouchPoint {
    float x;
    float y;
    // 方位角
    float azimuth_angle;
    // 倾斜角
    float altitude_angle;
    float force;
    float stamp;
    float distance;
    float interval;
    float speed;
    // -1: 无压感， 0: touch 结束点, 1: pencil, 2: 3D touch
    int ty;
    float stamp_scale;
    
};

struct RustByteSlice {
    const uint8_t *bytes;
    size_t len;
};

#endif /* rs_glue_h */
