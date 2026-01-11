use crate::views::html_view::HtmlView;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::Response;

/// 页面控制器
pub struct PageController;

impl PageController {
    /// 创建新的页面控制器
    pub fn new() -> Self {
        Self
    }

    /// 渲染首页
    pub fn render_home(&self) -> Response<Full<Bytes>> {
        HtmlView::render_home()
    }
}

impl Default for PageController {
    fn default() -> Self {
        Self::new()
    }
}
