//
//  MetalView.swift
//  vulkan_ios
//
//  Created by grenlight on 2018/11/23.
//

import UIKit
import Foundation

class MetalView: UIView {
    
    override class var layerClass: AnyClass {
        return CAMetalLayer.self
    }
    
    override func awakeFromNib() {
        super.awakeFromNib()
        configLayer()
    }
    
    private func configLayer() {
        guard let layer = self.layer as? CAMetalLayer else {
            return
        }

        layer.presentsWithTransaction = false
        // nativeScale 表示真实的物理屏幕缩放
        // https://tomisacat.xyz/tech/2017/06/17/scale-nativescale-contentsscale.html
        layer.contentsScale = UIScreen.main.nativeScale
        layer.removeAllAnimations()
    }
    
    func appView() -> app_view {
        self.contentScaleFactor = UIScreen.main.nativeScale
        let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())
        let metalLayer = UnsafeMutableRawPointer(Unmanaged.passRetained(self.layer).toOpaque())
        return app_view(view: ownedPointer, metal_layer: metalLayer)
    }
    
}
