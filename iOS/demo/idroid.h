//
//  idroid.h
//  brush
//
//  Created by grenlight on 2019/5/28.
//  Copyright © 2019 grenlight. All rights reserved.
//

#ifndef idroid_h
#define idroid_h

#include <stdint.h>

#include "rs_glue.h"

const char* rust_greeting(const char* to);
void rust_greeting_free(char *);

void give_object_to_rust(struct app_view object);


// Create a new instance of `rust_obj`.
struct idroid_obj *new_test_obj(void);

struct idroid_obj *create_triangle(struct app_view object);
struct idroid_obj *create_blur_filter(struct app_view object);
struct idroid_obj *create_gray_filter(struct app_view object);
struct idroid_obj *create_page_turning(struct app_view object);
struct idroid_obj *create_roll_animation(struct app_view object);
struct idroid_obj *create_brush_view(struct app_view object);
struct idroid_obj *create_fluid(struct app_view object);

void enter_frame(struct rust_obj *data);
void touch_move(struct rust_obj *data, struct TouchPoint p);

#endif /* idroid_h */
