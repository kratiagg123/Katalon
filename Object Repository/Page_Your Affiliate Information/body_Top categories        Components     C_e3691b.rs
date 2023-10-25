<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Top categories        Components     C_e3691b</name>
   <tag></tag>
   <elementGuidId>3958fc18-5e93-477b-8b96-3c85b81655f2</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//*/text()[normalize-space(.)='']/parent::*</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body.account-affiliate-add</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>6045b5a7-2166-4b2f-bbce-09614995772d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>account-affiliate-add</value>
      <webElementGuid>349a683d-d113-4aa4-b58e-6b86599a2412</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>   Top categories      
  Components
 

  
  Cameras
 

  
  Phone, Tablets &amp; Ipod
 

  
  Software
 

  
  MP3 Players
 

  
  Laptops &amp; Notebooks
 

  
  Desktops and Monitors
 

  
  Printers &amp; Scanners
 

  
  Mice and Trackballs
 

  
  Fashion and Accessories
 

  
  Beauty and Saloon
 

  
  Autoparts and Accessories
 

  
  Washing machine
 

  
  Gaming consoles
 

  
  Air conditioner
 

  
  Web Cameras
 

   Quick Links      
  Special
 
Hot 
  
  Wishlist
 

  
  Compare
 

  
  My account
 

  
  Blog
 

  
  Tracking
 

  
  Contact us
 

  Place here any module, widget, design or HTML. for example menu, categories  Cart         Palm Treo Pro Model: Product 2  x1 $279.99      iPod Shuffle Model: Product 7  x1 $150.00     Sub-Total: $429.99   Total: $429.99    Edit cart  Checkout 
                 All Categories  All Categories Desktops Laptops Components Tablets Software Phones &amp; PDAs Cameras MP3 Players
          Search          2   2 item(s) - $429.99          2   2 item(s) - $429.99    Shop by Category     
  Home
 

 
  Special
 
Hot 
 
  Blog
 

 
  Mega Menu
 

 
Mobiles

  
Apple
   
HTC
   
LG
   
Nokia
   
Samsung
   
Xiomi
   
Laptops

  
Apple Macbook
   
Asus
   
HP
   
Lenovo
   
Accessories

  
Headphones
   
Memory Card
   
Mobile cases
   
Power bank
   
Screenguards
   
Smart Wearable

  
Smart Watch
   
Smart band
   
Tablets

  
Apple Ipad
   
Computers

  
Desktop
   
Hard disk
   
Mouse &amp; Keyboard
   
Pen Drive
   
Printer
   
Sound System

  
Bluetooth Speaker
   
DTH
   
Home Audio
   
Home Theatre
   
SoundBar
        
  AddOns
 
Featured 
  
  Modules
 

 
  Designs
 

 
  Widgets
 

    
  My account
 

  
  Dashboard
 

 
  My order
 

 
  Return
 

 
  Tracking
 

 
  My voucher
 

 
  Logout
 

          All Categories  All Categories Desktops Laptops Components Tablets Software Phones &amp; PDAs Cameras MP3 Players
             This is a dummy website for Web Automation Testing Upto 60% Off on Smartbuy Mobile Accessories, Small Appliances, Automotive Accessories &amp; more SAVE60     Account Affiliate     Your Affiliate Information   My Affiliate Account  Company      Web Site       Payment Information  Tax ID      Payment Method     Cheque     PayPal     Bank Transfer     Cheque Payee Name      PayPal Email Account       Bank Name      ABA/BSB number (Branch Number)      SWIFT Code      Account Name      Account Number        I have read and agree to the About Us   
        My Account  Edit Account  Password  Address Book  Wish List  Notification  Order History  Downloads  Recurring payments  Reward Points  Returns  Transactions  Newsletter  Logout  &lt;!--
$('input[name=\'payment\']').on('change', function() {
$('.payment').hide();
$('#payment-' + this.value).show();
});
$('input[name=\'payment\']:checked').trigger('change');
//-->&lt;!--
// Sort the custom fields
$('.form-group[data-sort]').detach().each(function() {
if ($(this).attr('data-sort') >= 0 &amp;&amp; $(this).attr('data-sort') &lt;= $('.form-group').length) {
$('.form-group').eq($(this).attr('data-sort')).before(this);
}
if ($(this).attr('data-sort') > $('.form-group').length) {
$('.form-group:last').after(this);
}
if ($(this).attr('data-sort') == $('.form-group').length) {
$('.form-group:last').after(this);
}
if ($(this).attr('data-sort') &lt; -$('.form-group').length) {
$('.form-group:first').before(this);
}
});
//-->&lt;!--
$('button[id^=\'button-custom-field\']').on('click', function() {
var element = this;
$('#form-upload').remove();
$('body').prepend('&lt;form enctype=&quot;multipart/form-data&quot; id=&quot;form-upload&quot; style=&quot;display: none;&quot;>&lt;input type=&quot;file&quot; name=&quot;file&quot; />&lt;/form>');
$('#form-upload input[name=\'file\']').trigger('click');
if (typeof timer != 'undefined') {
clearInterval(timer);
}
timer = setInterval(function() {
if ($('#form-upload input[name=\'file\']').val() != '') {
clearInterval(timer);
$.ajax({
url: 'index.php?route=tool/upload',
type: 'post',
dataType: 'json',
data: new FormData($('#form-upload')[0]),
cache: false,
contentType: false,
processData: false,
beforeSend: function() {
$(element).button('loading');
},
complete: function() {
$(element).button('reset');
},
success: function(json) {
$(element).parent().find('.text-danger').remove();
if (json['error']) {
$(element).parent().find('input').after('&lt;div class=&quot;text-danger&quot;>' + json['error'] + '&lt;/div>');
}
if (json['success']) {
alert(json['success']);
$(element).parent().find('input').val(json['code']);
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
}
}, 500);
});
//-->&lt;!--
$(function(){
$('.date, .time, .datetime').datetimepicker({
locale: 'en-gb',
icons: {
time: &quot;far fa-clock&quot;,
date: &quot;far fa-calendar-alt&quot;,
up: &quot;fas fa-arrow-up&quot;,
down: &quot;fas fa-arrow-down&quot;
}
});
});
//-->© LambdaTest - Powered by OpenCartmenu $('#back-to-top').click(function(e){
e.preventDefault();$('html, body').animate({scrollTop: 0}, 800);
});
window.addEventListener(&quot;scroll&quot;, function(){
var el = $('#back-to-top');
if((window.pageYOffset > window.innerHeight) &amp;&amp; !el.data('show')){
el.data('show', 1); el.fadeIn();
} else if((window.pageYOffset &lt;= window.innerHeight) &amp;&amp; el.data('show')){
el.data('show', 0); el.fadeOut();
}
});id(&quot;input-company&quot;)</value>
      <webElementGuid>fc8983e8-6ba1-43de-9610-1d2fe2a48a2c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;account-affiliate-add&quot;]</value>
      <webElementGuid>4f125c1f-220f-44de-8d2f-42f5c06b8c67</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
      <webElementGuid>cd44f6df-41fd-4092-b12e-d743fd0bbdd3</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>2e6fcdcf-af15-4d4a-bdb1-e2cab2b2ab68</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;   Top categories      
  Components
 

  
  Cameras
 

  
  Phone, Tablets &amp; Ipod
 

  
  Software
 

  
  MP3 Players
 

  
  Laptops &amp; Notebooks
 

  
  Desktops and Monitors
 

  
  Printers &amp; Scanners
 

  
  Mice and Trackballs
 

  
  Fashion and Accessories
 

  
  Beauty and Saloon
 

  
  Autoparts and Accessories
 

  
  Washing machine
 

  
  Gaming consoles
 

  
  Air conditioner
 

  
  Web Cameras
 

   Quick Links      
  Special
 
Hot 
  
  Wishlist
 

  
  Compare
 

  
  My account
 

  
  Blog
 

  
  Tracking
 

  
  Contact us
 

  Place here any module, widget, design or HTML. for example menu, categories  Cart         Palm Treo Pro Model: Product 2  x1 $279.99      iPod Shuffle Model: Product 7  x1 $150.00     Sub-Total: $429.99   Total: $429.99    Edit cart  Checkout 
                 All Categories  All Categories Desktops Laptops Components Tablets Software Phones &amp; PDAs Cameras MP3 Players
          Search          2   2 item(s) - $429.99          2   2 item(s) - $429.99    Shop by Category     
  Home
 

 
  Special
 
Hot 
 
  Blog
 

 
  Mega Menu
 

 
Mobiles

  
Apple
   
HTC
   
LG
   
Nokia
   
Samsung
   
Xiomi
   
Laptops

  
Apple Macbook
   
Asus
   
HP
   
Lenovo
   
Accessories

  
Headphones
   
Memory Card
   
Mobile cases
   
Power bank
   
Screenguards
   
Smart Wearable

  
Smart Watch
   
Smart band
   
Tablets

  
Apple Ipad
   
Computers

  
Desktop
   
Hard disk
   
Mouse &amp; Keyboard
   
Pen Drive
   
Printer
   
Sound System

  
Bluetooth Speaker
   
DTH
   
Home Audio
   
Home Theatre
   
SoundBar
        
  AddOns
 
Featured 
  
  Modules
 

 
  Designs
 

 
  Widgets
 

    
  My account
 

  
  Dashboard
 

 
  My order
 

 
  Return
 

 
  Tracking
 

 
  My voucher
 

 
  Logout
 

          All Categories  All Categories Desktops Laptops Components Tablets Software Phones &amp; PDAs Cameras MP3 Players
             This is a dummy website for Web Automation Testing Upto 60% Off on Smartbuy Mobile Accessories, Small Appliances, Automotive Accessories &amp; more SAVE60     Account Affiliate     Your Affiliate Information   My Affiliate Account  Company      Web Site       Payment Information  Tax ID      Payment Method     Cheque     PayPal     Bank Transfer     Cheque Payee Name      PayPal Email Account       Bank Name      ABA/BSB number (Branch Number)      SWIFT Code      Account Name      Account Number        I have read and agree to the About Us   
        My Account  Edit Account  Password  Address Book  Wish List  Notification  Order History  Downloads  Recurring payments  Reward Points  Returns  Transactions  Newsletter  Logout  &lt;!--
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;payment\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
$(&quot; , &quot;'&quot; , &quot;.payment&quot; , &quot;'&quot; , &quot;).hide();
$(&quot; , &quot;'&quot; , &quot;#payment-&quot; , &quot;'&quot; , &quot; + this.value).show();
});
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;payment\&quot; , &quot;'&quot; , &quot;]:checked&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
//-->&lt;!--
// Sort the custom fields
$(&quot; , &quot;'&quot; , &quot;.form-group[data-sort]&quot; , &quot;'&quot; , &quot;).detach().each(function() {
if ($(this).attr(&quot; , &quot;'&quot; , &quot;data-sort&quot; , &quot;'&quot; , &quot;) >= 0 &amp;&amp; $(this).attr(&quot; , &quot;'&quot; , &quot;data-sort&quot; , &quot;'&quot; , &quot;) &lt;= $(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).length) {
$(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).eq($(this).attr(&quot; , &quot;'&quot; , &quot;data-sort&quot; , &quot;'&quot; , &quot;)).before(this);
}
if ($(this).attr(&quot; , &quot;'&quot; , &quot;data-sort&quot; , &quot;'&quot; , &quot;) > $(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).length) {
$(&quot; , &quot;'&quot; , &quot;.form-group:last&quot; , &quot;'&quot; , &quot;).after(this);
}
if ($(this).attr(&quot; , &quot;'&quot; , &quot;data-sort&quot; , &quot;'&quot; , &quot;) == $(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).length) {
$(&quot; , &quot;'&quot; , &quot;.form-group:last&quot; , &quot;'&quot; , &quot;).after(this);
}
if ($(this).attr(&quot; , &quot;'&quot; , &quot;data-sort&quot; , &quot;'&quot; , &quot;) &lt; -$(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).length) {
$(&quot; , &quot;'&quot; , &quot;.form-group:first&quot; , &quot;'&quot; , &quot;).before(this);
}
});
//-->&lt;!--
$(&quot; , &quot;'&quot; , &quot;button[id^=\&quot; , &quot;'&quot; , &quot;button-custom-field\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function() {
var element = this;
$(&quot; , &quot;'&quot; , &quot;#form-upload&quot; , &quot;'&quot; , &quot;).remove();
$(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).prepend(&quot; , &quot;'&quot; , &quot;&lt;form enctype=&quot;multipart/form-data&quot; id=&quot;form-upload&quot; style=&quot;display: none;&quot;>&lt;input type=&quot;file&quot; name=&quot;file&quot; />&lt;/form>&quot; , &quot;'&quot; , &quot;);
$(&quot; , &quot;'&quot; , &quot;#form-upload input[name=\&quot; , &quot;'&quot; , &quot;file\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;);
if (typeof timer != &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
clearInterval(timer);
}
timer = setInterval(function() {
if ($(&quot; , &quot;'&quot; , &quot;#form-upload input[name=\&quot; , &quot;'&quot; , &quot;file\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
clearInterval(timer);
$.ajax({
url: &quot; , &quot;'&quot; , &quot;index.php?route=tool/upload&quot; , &quot;'&quot; , &quot;,
type: &quot; , &quot;'&quot; , &quot;post&quot; , &quot;'&quot; , &quot;,
dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
data: new FormData($(&quot; , &quot;'&quot; , &quot;#form-upload&quot; , &quot;'&quot; , &quot;)[0]),
cache: false,
contentType: false,
processData: false,
beforeSend: function() {
$(element).button(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
},
complete: function() {
$(element).button(&quot; , &quot;'&quot; , &quot;reset&quot; , &quot;'&quot; , &quot;);
},
success: function(json) {
$(element).parent().find(&quot; , &quot;'&quot; , &quot;.text-danger&quot; , &quot;'&quot; , &quot;).remove();
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;]) {
$(element).parent().find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).after(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;text-danger&quot;>&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;]) {
alert(json[&quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;]);
$(element).parent().find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).val(json[&quot; , &quot;'&quot; , &quot;code&quot; , &quot;'&quot; , &quot;]);
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
}
}, 500);
});
//-->&lt;!--
$(function(){
$(&quot; , &quot;'&quot; , &quot;.date, .time, .datetime&quot; , &quot;'&quot; , &quot;).datetimepicker({
locale: &quot; , &quot;'&quot; , &quot;en-gb&quot; , &quot;'&quot; , &quot;,
icons: {
time: &quot;far fa-clock&quot;,
date: &quot;far fa-calendar-alt&quot;,
up: &quot;fas fa-arrow-up&quot;,
down: &quot;fas fa-arrow-down&quot;
}
});
});
//-->© LambdaTest - Powered by OpenCartmenu $(&quot; , &quot;'&quot; , &quot;#back-to-top&quot; , &quot;'&quot; , &quot;).click(function(e){
e.preventDefault();$(&quot; , &quot;'&quot; , &quot;html, body&quot; , &quot;'&quot; , &quot;).animate({scrollTop: 0}, 800);
});
window.addEventListener(&quot;scroll&quot;, function(){
var el = $(&quot; , &quot;'&quot; , &quot;#back-to-top&quot; , &quot;'&quot; , &quot;);
if((window.pageYOffset > window.innerHeight) &amp;&amp; !el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;)){
el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;, 1); el.fadeIn();
} else if((window.pageYOffset &lt;= window.innerHeight) &amp;&amp; el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;)){
el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;, 0); el.fadeOut();
}
});id(&quot;input-company&quot;)&quot;) or . = concat(&quot;   Top categories      
  Components
 

  
  Cameras
 

  
  Phone, Tablets &amp; Ipod
 

  
  Software
 

  
  MP3 Players
 

  
  Laptops &amp; Notebooks
 

  
  Desktops and Monitors
 

  
  Printers &amp; Scanners
 

  
  Mice and Trackballs
 

  
  Fashion and Accessories
 

  
  Beauty and Saloon
 

  
  Autoparts and Accessories
 

  
  Washing machine
 

  
  Gaming consoles
 

  
  Air conditioner
 

  
  Web Cameras
 

   Quick Links      
  Special
 
Hot 
  
  Wishlist
 

  
  Compare
 

  
  My account
 

  
  Blog
 

  
  Tracking
 

  
  Contact us
 

  Place here any module, widget, design or HTML. for example menu, categories  Cart         Palm Treo Pro Model: Product 2  x1 $279.99      iPod Shuffle Model: Product 7  x1 $150.00     Sub-Total: $429.99   Total: $429.99    Edit cart  Checkout 
                 All Categories  All Categories Desktops Laptops Components Tablets Software Phones &amp; PDAs Cameras MP3 Players
          Search          2   2 item(s) - $429.99          2   2 item(s) - $429.99    Shop by Category     
  Home
 

 
  Special
 
Hot 
 
  Blog
 

 
  Mega Menu
 

 
Mobiles

  
Apple
   
HTC
   
LG
   
Nokia
   
Samsung
   
Xiomi
   
Laptops

  
Apple Macbook
   
Asus
   
HP
   
Lenovo
   
Accessories

  
Headphones
   
Memory Card
   
Mobile cases
   
Power bank
   
Screenguards
   
Smart Wearable

  
Smart Watch
   
Smart band
   
Tablets

  
Apple Ipad
   
Computers

  
Desktop
   
Hard disk
   
Mouse &amp; Keyboard
   
Pen Drive
   
Printer
   
Sound System

  
Bluetooth Speaker
   
DTH
   
Home Audio
   
Home Theatre
   
SoundBar
        
  AddOns
 
Featured 
  
  Modules
 

 
  Designs
 

 
  Widgets
 

    
  My account
 

  
  Dashboard
 

 
  My order
 

 
  Return
 

 
  Tracking
 

 
  My voucher
 

 
  Logout
 

          All Categories  All Categories Desktops Laptops Components Tablets Software Phones &amp; PDAs Cameras MP3 Players
             This is a dummy website for Web Automation Testing Upto 60% Off on Smartbuy Mobile Accessories, Small Appliances, Automotive Accessories &amp; more SAVE60     Account Affiliate     Your Affiliate Information   My Affiliate Account  Company      Web Site       Payment Information  Tax ID      Payment Method     Cheque     PayPal     Bank Transfer     Cheque Payee Name      PayPal Email Account       Bank Name      ABA/BSB number (Branch Number)      SWIFT Code      Account Name      Account Number        I have read and agree to the About Us   
        My Account  Edit Account  Password  Address Book  Wish List  Notification  Order History  Downloads  Recurring payments  Reward Points  Returns  Transactions  Newsletter  Logout  &lt;!--
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;payment\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
$(&quot; , &quot;'&quot; , &quot;.payment&quot; , &quot;'&quot; , &quot;).hide();
$(&quot; , &quot;'&quot; , &quot;#payment-&quot; , &quot;'&quot; , &quot; + this.value).show();
});
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;payment\&quot; , &quot;'&quot; , &quot;]:checked&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
//-->&lt;!--
// Sort the custom fields
$(&quot; , &quot;'&quot; , &quot;.form-group[data-sort]&quot; , &quot;'&quot; , &quot;).detach().each(function() {
if ($(this).attr(&quot; , &quot;'&quot; , &quot;data-sort&quot; , &quot;'&quot; , &quot;) >= 0 &amp;&amp; $(this).attr(&quot; , &quot;'&quot; , &quot;data-sort&quot; , &quot;'&quot; , &quot;) &lt;= $(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).length) {
$(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).eq($(this).attr(&quot; , &quot;'&quot; , &quot;data-sort&quot; , &quot;'&quot; , &quot;)).before(this);
}
if ($(this).attr(&quot; , &quot;'&quot; , &quot;data-sort&quot; , &quot;'&quot; , &quot;) > $(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).length) {
$(&quot; , &quot;'&quot; , &quot;.form-group:last&quot; , &quot;'&quot; , &quot;).after(this);
}
if ($(this).attr(&quot; , &quot;'&quot; , &quot;data-sort&quot; , &quot;'&quot; , &quot;) == $(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).length) {
$(&quot; , &quot;'&quot; , &quot;.form-group:last&quot; , &quot;'&quot; , &quot;).after(this);
}
if ($(this).attr(&quot; , &quot;'&quot; , &quot;data-sort&quot; , &quot;'&quot; , &quot;) &lt; -$(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).length) {
$(&quot; , &quot;'&quot; , &quot;.form-group:first&quot; , &quot;'&quot; , &quot;).before(this);
}
});
//-->&lt;!--
$(&quot; , &quot;'&quot; , &quot;button[id^=\&quot; , &quot;'&quot; , &quot;button-custom-field\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function() {
var element = this;
$(&quot; , &quot;'&quot; , &quot;#form-upload&quot; , &quot;'&quot; , &quot;).remove();
$(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).prepend(&quot; , &quot;'&quot; , &quot;&lt;form enctype=&quot;multipart/form-data&quot; id=&quot;form-upload&quot; style=&quot;display: none;&quot;>&lt;input type=&quot;file&quot; name=&quot;file&quot; />&lt;/form>&quot; , &quot;'&quot; , &quot;);
$(&quot; , &quot;'&quot; , &quot;#form-upload input[name=\&quot; , &quot;'&quot; , &quot;file\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;);
if (typeof timer != &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
clearInterval(timer);
}
timer = setInterval(function() {
if ($(&quot; , &quot;'&quot; , &quot;#form-upload input[name=\&quot; , &quot;'&quot; , &quot;file\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
clearInterval(timer);
$.ajax({
url: &quot; , &quot;'&quot; , &quot;index.php?route=tool/upload&quot; , &quot;'&quot; , &quot;,
type: &quot; , &quot;'&quot; , &quot;post&quot; , &quot;'&quot; , &quot;,
dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
data: new FormData($(&quot; , &quot;'&quot; , &quot;#form-upload&quot; , &quot;'&quot; , &quot;)[0]),
cache: false,
contentType: false,
processData: false,
beforeSend: function() {
$(element).button(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
},
complete: function() {
$(element).button(&quot; , &quot;'&quot; , &quot;reset&quot; , &quot;'&quot; , &quot;);
},
success: function(json) {
$(element).parent().find(&quot; , &quot;'&quot; , &quot;.text-danger&quot; , &quot;'&quot; , &quot;).remove();
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;]) {
$(element).parent().find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).after(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;text-danger&quot;>&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;]) {
alert(json[&quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;]);
$(element).parent().find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).val(json[&quot; , &quot;'&quot; , &quot;code&quot; , &quot;'&quot; , &quot;]);
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
}
}, 500);
});
//-->&lt;!--
$(function(){
$(&quot; , &quot;'&quot; , &quot;.date, .time, .datetime&quot; , &quot;'&quot; , &quot;).datetimepicker({
locale: &quot; , &quot;'&quot; , &quot;en-gb&quot; , &quot;'&quot; , &quot;,
icons: {
time: &quot;far fa-clock&quot;,
date: &quot;far fa-calendar-alt&quot;,
up: &quot;fas fa-arrow-up&quot;,
down: &quot;fas fa-arrow-down&quot;
}
});
});
//-->© LambdaTest - Powered by OpenCartmenu $(&quot; , &quot;'&quot; , &quot;#back-to-top&quot; , &quot;'&quot; , &quot;).click(function(e){
e.preventDefault();$(&quot; , &quot;'&quot; , &quot;html, body&quot; , &quot;'&quot; , &quot;).animate({scrollTop: 0}, 800);
});
window.addEventListener(&quot;scroll&quot;, function(){
var el = $(&quot; , &quot;'&quot; , &quot;#back-to-top&quot; , &quot;'&quot; , &quot;);
if((window.pageYOffset > window.innerHeight) &amp;&amp; !el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;)){
el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;, 1); el.fadeIn();
} else if((window.pageYOffset &lt;= window.innerHeight) &amp;&amp; el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;)){
el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;, 0); el.fadeOut();
}
});id(&quot;input-company&quot;)&quot;))]</value>
      <webElementGuid>b402127f-ef58-4e03-b396-0003aaf8742c</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
