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


struct idroid_obj *create_sdf_view(struct app_view object);
void sdf_view_bundle_image(struct rust_obj *data, char* image_name);

void enter_frame(struct rust_obj *data);
void touch_move(struct rust_obj *data, struct TouchPoint p);
void resize(struct rust_obj *data);
void scale(struct rust_obj *data, float scale);

#endif /* idroid_h */
