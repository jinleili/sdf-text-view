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
//   z float (*screen_scale)(void);
//    struct view_size (*get_inner_size)(void);
};

struct view_size {
    int width;
    int height;
};

struct TouchPoint {
    float x;
    float y;
    float force;
};

struct RustByteSlice {
    const uint8_t *bytes;
    size_t len;
};

#endif /* rs_glue_h */
