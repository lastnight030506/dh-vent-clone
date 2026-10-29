# Cài đặt VENT trên Windows

Hướng dẫn dành cho người dùng không chuyên. Không cần cài Rust, Node.js hay bất kỳ công cụ lập trình nào.

## 1. Tải bản cài đặt

Mở trang tải xuống:

**https://github.com/lastnight030506/dh-vent-clone/releases/latest**

Ở mục **Assets** (phía dưới phần mô tả), bấm tệp:

```
VENT_1.0.0_x64-setup.exe
```

Đây là bộ cài Windows 64-bit. Nếu không thấy tệp này, bấm **Show all assets** để mở rộng danh sách.

> Nếu trang báo **404** hoặc yêu cầu đăng nhập GitHub, tài liệu đang nằm trong kho riêng tư. Hãy liên hệ người gửi để được cấp quyền, hoặc nhận trực tiếp tệp `.exe` qua USB/email.

## 2. Cài đặt

1. Mở tệp `VENT_1.0.0_x64-setup.exe` vừa tải.
2. Nếu Windows hiện hộp thoại xanh **"Windows protected your PC"**:
   - bấm **More info** (Thông tin thêm),
   - rồi bấm **Run anyway** (Vẫn chạy).
   - Lý do: bản cài chưa có chữ ký số trả phí, không phải vì tệp có virus. Chỉ chạy tệp bạn tải từ đúng trang Releases ở trên.
3. Bấm **Next** → chọn thư mục cài (mặc định là đủ) → **Install** → **Finish**.
4. Mở **VENT** từ Start Menu hoặc biểu tượng ngoài Desktop.

## 3. Yêu cầu

- Windows 10 hoặc Windows 11, bản 64-bit.
- Lần cài đầu tiên cần Internet để tải **Microsoft Edge WebView2** nếu máy chưa có. Sau đó app chạy hoàn toàn offline.
- Không cần mã truy cập, không cần đăng ký tài khoản. Toàn bộ tính toán chạy trên máy.

## 4. Gói .msi (dành cho quản trị viên)

Trong cùng mục **Assets** có tệp:

```
VENT_1.0.0_x64_en-US.msi
```

Dùng gói này nếu bạn cài hàng loạt bằng Group Policy hoặc cần quản lý cài đặt tập trung. Người dùng thường nên dùng tệp `.exe` ở bước 1.

## 5. Cập nhật

Tải bản cài mới ở trang Releases và chạy như lần đầu. Bản mới cài đè lên bản cũ; dữ liệu dự án lưu trong trình duyệt của app vẫn giữ nguyên.

## 6. Gỡ cài đặt

**Settings → Apps → Installed apps → VENT → Uninstall**.

## 7. Xử lý sự cố

| Hiện tượng | Cách xử lý |
| --- | --- |
| SmartScreen chặn | **More info → Run anyway** (xem bước 2). |
| Cài xong nhưng app không mở | Cài **Microsoft Edge WebView2 Runtime** từ trang chủ Microsoft, rồi mở lại VENT. |
| Trình duyệt tải tệp về dạng `.exe.txt` | Đổi tên bỏ đuôi `.txt`, còn lại `.exe`, rồi chạy. |
| Trang Releases báo 404 | Kho đang riêng tư – liên hệ người gửi để được cấp quyền hoặc nhận tệp trực tiếp. |

---

# Install VENT on Windows (English)

1. Open **https://github.com/lastnight030506/dh-vent-clone/releases/latest**.
2. Under **Assets**, download `VENT_1.0.0_x64-setup.exe`.
3. Run it. If SmartScreen appears, choose **More info → Run anyway** (the installer is not code-signed).
4. Finish the installer and launch **VENT** from the Start Menu.

Requires Windows 10/11 64-bit. Internet is needed once if WebView2 is missing; after that the app is fully offline. No access code or account. The `.msi` in the same Assets section is for system administrators.
