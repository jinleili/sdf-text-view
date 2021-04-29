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
void sdf_view_set_bundle_image(struct idroid_obj *data, char* image_name);

void enter_frame(struct idroid_obj *data);
void touch_move(struct idroid_obj *data, struct TouchPoint p);
void resize(struct idroid_obj *data);
void pintch_changed(struct idroid_obj *data, struct TouchPoint location, float scale);

#endif /* idroid_h */
