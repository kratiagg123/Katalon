<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Top categories        Components     C_e3691b</name>
   <tag></tag>
   <elementGuidId>79a4f5e2-e3d9-4bf0-bb69-fc456e018189</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//*/text()[normalize-space(.)='']/parent::*</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body.product-category</value>
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
      <webElementGuid>5d02a7ef-ae49-4e12-bfec-411e15cd5c51</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>product-category</value>
      <webElementGuid>3cc932ef-8a13-48d6-9b1a-404b74a86c5c</webElementGuid>
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
 

  Place here any module, widget, design or HTML. for example menu, categories  Cart  Your shopping cart is empty!  Sub-Total: $0.00   Total: $0.00    Edit cart  Checkout   Filter   
  Price
        to     
  Manufacturer
  

  


Apple
 
42
 


Canon
 
10
 


Hewlett-Packard
 
10
 


HTC
 
8
 


Nikon
 
2
 


Palm
 
2
 


Sony
 
1
 See more   
  Search
        
  Color
      
 

   
 

   
 

   
 

   
 

   
 

   
 

   
 

     
  Availability
      

In stock 
73
 

Pre-Order 
2
   
  Size
      
 
L
   
 
M
   
 
S
   
 
XL
   
 
XXL
     
  Discount
    
 
 10% off or more  0

 
 20% off or more  0

 
 30% off or more  0

 
 40% off or more  0

 
 50% off or more  0
   
  Rating
    
 

        &amp; up
  0

 

        &amp; up
  0

 

        &amp; up
  0

 

        &amp; up
  0

  
$(function(){
// hide refine search if sub category filter selected
if($('[name=&quot;mz_fsc&quot;]:checked').length || $('[name=&quot;mz_fq&quot;]').val()){
$('.content-refine-search').attr('style', 'display: none !important');
}
// Filter
var paginationContainer = $('.content-pagination');
var productContainer = $('.content-products');
$('#mz-filter-1').mz_filter({
requestURL: &quot;index.php?route=extension/module/mz_filter/product&amp;path=57&amp;sub_category=1&amp;_count_product&amp;_f_manufacturer&amp;_f_stock_status&amp;_f_rating&amp;_f_discount&amp;_filter_require_category&amp;_f_custom&amp;target_route=product/category&amp;module_id=95&amp;product_entry_id=&quot; + $('.content-products').data('id') + &quot;&amp;pagination_entry_id=&quot; + $('.content-pagination').data('id'),
searchEl: $('#mz-filter-1 .mz-filter-group-search input'),
ajax: true,
delay: true,
hideZeroFilter: false,
search_in_description: true,
countProduct: true,
sortBy: 'product',
onParamChange: function(param){
$(&quot;.content-limit,.content-sort-by&quot;).find('option').each(function(){
var url = $(this).attr('value');
$(this).attr('value', modifyURLQuery(url, $.extend({}, param, {page: null})));
});
var currency = $('#form-currency input[name=&quot;redirect&quot;]');
currency.val(modifyURLQuery(currency.val(), $.extend({}, param, {mz_fp: null, page: null})));
// Show or hide reset all button
if($('#mz-filter-1 .mz-filter-group [data-mz-reset]:not(.d-none)').length){
$('#mz-filter-1 [data-mz-reset=&quot;all&quot;]').removeClass('d-none');
} else {
$('#mz-filter-1 [data-mz-reset=&quot;all&quot;]').addClass('d-none');
}
},
onInputChange: function(e){
var filter_group = $(e.target).closest('.mz-filter-group');
var is_input_selected = false;
// Hide Reset for Checkbox or radio
if(filter_group.find('input[type=&quot;checkbox&quot;]:checked,input[type=&quot;radio&quot;]:checked').length){
is_input_selected = true;
}
// Hide Reset for price
if($(e.target).filter('[name=&quot;mz_fp[min]&quot;],[name=&quot;mz_fp[max]&quot;]').length){
if($('#mz-filter-1 [name=&quot;mz_fp[min]&quot;]').val() !== $('#mz-filter-1 [name=&quot;mz_fp[min]&quot;]').attr('min') || $('#mz-filter-1 [name=&quot;mz_fp[max]&quot;]').val() !== $('#mz-filter-1 [name=&quot;mz_fp[max]&quot;]').attr('max')){
is_input_selected = true;
}
}
// Hide reset for text
if($(e.target).filter('[type=&quot;text&quot;]').val()){
is_input_selected = true;
}
// Hide or show reset buton
if(is_input_selected){
filter_group.find('[data-mz-reset]').removeClass('d-none');
} else {
filter_group.find('[data-mz-reset]').addClass('d-none');
}
},
onReset: function(type){
// Reset price
if(type === 'price' || type === 'all'){
price_slider.slider(&quot;values&quot;, [parseFloat(price_slider.slider(&quot;option&quot;, 'min')), parseFloat(price_slider.slider(&quot;option&quot;, 'max'))]);
}
},
onBeforeSend: function(){
productContainer.append('&lt;div class=&quot;mz-filter-loader&quot;>&lt;div class=&quot;loader-spinner&quot;>&lt;/div>&lt;/div>');
},
onResult: function(json, mz_filter){
// Add result products to container
if(json['products']){
productContainer.find('[data-countdown]').countdown('destroy');
productContainer.html(json['products']);
pageLoad(productContainer);
productContainer.find('#button-continue').on('click', function(e){
e.preventDefault();
mz_filter.resetAll();
});
$('#list-view.active').click();
$('#grid-view.active').click();
} else {
productContainer.html(&quot;&lt;div class='col-xs-12 text-center'>No products found!&lt;/div>&quot;);
}
// Add pagination to container
if(json['pagination']){
paginationContainer.html(json['pagination']);
} else {
paginationContainer.empty();
}
// hide refine search if sub category filter selected
if(mz_filter.inputs.filter('[name=&quot;mz_fsc&quot;]:checked').length || mz_filter.inputs.filter('[name=&quot;mz_fq&quot;]').val()){
$('.content-refine-search').attr('style', 'display: none !important');
} else {
$('.content-refine-search').attr('style', '');
}
}
});
// Price slider
var price_slider = $(&quot;#mz-filter-1 [data-role='rangeslider']&quot;).slider({
range: true,
min: parseFloat($('#mz-filter-1 [name=&quot;mz_fp[min]&quot;]').attr('min')),
max: parseFloat($('#mz-filter-1 [name=&quot;mz_fp[max]&quot;]').attr('max')),
values: [parseFloat($('#mz-filter-1 [name=&quot;mz_fp[min]&quot;]').val()), parseFloat($('#mz-filter-1 [name=&quot;mz_fp[max]&quot;]').val())],
slide: function( event, ui ) {
$('#mz-filter-1 [name=&quot;mz_fp[min]&quot;]').val(ui.values[0]);
$('#mz-filter-1 [name=&quot;mz_fp[max]&quot;]').val(ui.values[1]);
},
change: function( event, ui ) {
// Hide Reset for price
if($('#mz-filter-1 [name=&quot;mz_fp[min]&quot;]').val() !== $('#mz-filter-1 [name=&quot;mz_fp[min]&quot;]').attr('min') || $('#mz-filter-1 [name=&quot;mz_fp[max]&quot;]').val() !== $('#mz-filter-1 [name=&quot;mz_fp[max]&quot;]').attr('max')){
$('#mz-filter-1 [data-mz-reset=&quot;price&quot;]').removeClass('d-none');
} else {
$('#mz-filter-1 [data-mz-reset=&quot;price&quot;]').addClass('d-none');
}
// Trigger filter change
$('#mz-filter-1').change();
}
});
$('#mz-filter-1 [name=&quot;mz_fp[min]&quot;]').change(function(){
price_slider.slider(&quot;values&quot;, 0, $(this).val());
});
$('#mz-filter-1 [name=&quot;mz_fp[max]&quot;]').change(function(){
price_slider.slider(&quot;values&quot;, 1, $(this).val());
});
// Show reset all button if filter is selected
if($('#mz-filter-1 [data-mz-reset]:not(.d-none)').length){
$('[data-mz-reset=&quot;all&quot;]').removeClass('d-none');
}
});                   All Categories  All Categories Desktops Laptops Components Tablets Software Phones &amp; PDAs Cameras MP3 Players
          Search          0   0 item(s) - $0.00          0   0 item(s) - $0.00    Shop by Category     
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
             This is a dummy website for Web Automation Testing Upto 60% Off on Smartbuy Mobile Accessories, Small Appliances, Automotive Accessories &amp; more SAVE60      Tablets Tablets
 Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Semper auctor neque vitae tempus quam pellentesque. Facilisis leo vel fringilla est ullamcorper. Tellus id interdum velit laoreet id donec ultrices tincidunt. Vitae nunc sed velit dignissim sodales. Mi proin sed libero enim sed. Mi quis hendrerit dolor magna eget est lorem ipsum.  Product Compare (0)  Filter Show:  15 25 50 75 100   Sort By:  Default Best sellers Popular Newest Name (A - Z) Name (Z - A) Price (Low > High) Price (High > Low) Rating (Highest) Rating (Lowest) Model (A - Z) Model (Z - A)   Show:  15 25 50 75 100   Sort By:  Default Best sellers Popular Newest Name (A - Z) Name (Z - A) Price (Low > High) Price (High > Low) Rating (Highest) Rating (Lowest) Model (A - Z) Model (Z - A)  There are no products to list in this category.

 Continue

  Filter    
  Price
        to     
  Manufacturer
  

    



Apple
 
0
 



Canon
 
0
 



Hewlett-Packard
 
0
 



HTC
 
0
 



Nikon
 
0
 



Palm
 
0
 



Sony
 
0
See more  
  Search
        
  Color
       
 

   
 

   
 

   
 

   
 

   
 

   
 

   
 

    
  Availability
       

In stock 
0
 

Pre-Order 
0
  
  Size
       
 
L
   
 
M
   
 
S
   
 
XL
   
 
XXL
    
  Discount
    
 
 10% off or more  0

 
 20% off or more  0

 
 30% off or more  0

 
 40% off or more  0

 
 50% off or more  0
   
  Rating
    
 

        &amp; up
  0

 

        &amp; up
  0

 

        &amp; up
  0

 

        &amp; up
  0

  
$(function(){
if(window.innerWidth &lt; 767){ // Collaped all panel in small device
$('#mz-filter-0 .collapse.show').collapse(&quot;hide&quot;);
}
// hide refine search if sub category filter selected
if($('[name=&quot;mz_fsc&quot;]:checked').length || $('[name=&quot;mz_fq&quot;]').val()){
$('.content-refine-search').attr('style', 'display: none !important');
}
// Filter
var paginationContainer = $('.content-pagination');
var productContainer = $('.content-products');
$('#mz-filter-0').mz_filter({
requestURL: &quot;index.php?route=extension/module/mz_filter/product&amp;path=57&amp;sub_category=1&amp;_count_product&amp;_f_manufacturer&amp;_f_sub_category&amp;_f_stock_status&amp;_f_rating&amp;_f_discount&amp;_filter_require_category&amp;_f_custom&amp;target_route=product/category&amp;module_id=80&amp;product_entry_id=&quot; + $('.content-products').data('id') + &quot;&amp;pagination_entry_id=&quot; + $('.content-pagination').data('id'),
searchEl: $('#mz-filter-0 .mz-filter-group-search input'),
ajax: true,
delay: true,
hideZeroFilter: false,
search_in_description: true,
countProduct: true,
sortBy: 'product',
onParamChange: function(param){
$(&quot;.content-limit,.content-sort-by&quot;).find('option').each(function(){
var url = $(this).attr('value');
$(this).attr('value', modifyURLQuery(url, $.extend({}, param, {page: null})));
});
var currency = $('#form-currency input[name=&quot;redirect&quot;]');
currency.val(modifyURLQuery(currency.val(), $.extend({}, param, {mz_fp: null, page: null})));
// Show or hide reset all button
if($('#mz-filter-0 .mz-filter-group [data-mz-reset]:not(.d-none)').length){
$('#mz-filter-0 [data-mz-reset=&quot;all&quot;]').removeClass('d-none');
} else {
$('#mz-filter-0 [data-mz-reset=&quot;all&quot;]').addClass('d-none');
}
},
onInputChange: function(e){
var filter_group = $(e.target).closest('.mz-filter-group');
var is_input_selected = false;
// Hide Reset for Checkbox or radio
if(filter_group.find('input[type=&quot;checkbox&quot;]:checked,input[type=&quot;radio&quot;]:checked').length){
is_input_selected = true;
}
// Hide Reset for price
if($(e.target).filter('[name=&quot;mz_fp[min]&quot;],[name=&quot;mz_fp[max]&quot;]').length){
if($('#mz-filter-0 [name=&quot;mz_fp[min]&quot;]').val() !== $('#mz-filter-0 [name=&quot;mz_fp[min]&quot;]').attr('min') || $('#mz-filter-0 [name=&quot;mz_fp[max]&quot;]').val() !== $('#mz-filter-0 [name=&quot;mz_fp[max]&quot;]').attr('max')){
is_input_selected = true;
}
}
// Hide reset for text
if($(e.target).filter('[type=&quot;text&quot;]').val()){
is_input_selected = true;
}
// Hide or show reset buton
if(is_input_selected){
filter_group.find('[data-mz-reset]').removeClass('d-none');
} else {
filter_group.find('[data-mz-reset]').addClass('d-none');
}
},
onReset: function(type){
// Reset price
if(type === 'price' || type === 'all'){
price_slider.slider(&quot;values&quot;, [parseFloat(price_slider.slider(&quot;option&quot;, 'min')), parseFloat(price_slider.slider(&quot;option&quot;, 'max'))]);
}
},
onBeforeSend: function(){
productContainer.append('&lt;div class=&quot;mz-filter-loader&quot;>&lt;div class=&quot;loader-spinner&quot;>&lt;/div>&lt;/div>');
},
onResult: function(json, mz_filter){
// Add result products to container
if(json['products']){
productContainer.find('[data-countdown]').countdown('destroy');
productContainer.html(json['products']);
pageLoad(productContainer);
productContainer.find('#button-continue').on('click', function(e){
e.preventDefault();
mz_filter.resetAll();
});
$('#list-view.active').click();
$('#grid-view.active').click();
} else {
productContainer.html(&quot;&lt;div class='col-xs-12 text-center'>No products found!&lt;/div>&quot;);
}
// Add pagination to container
if(json['pagination']){
paginationContainer.html(json['pagination']);
} else {
paginationContainer.empty();
}
// hide refine search if sub category filter selected
if(mz_filter.inputs.filter('[name=&quot;mz_fsc&quot;]:checked').length || mz_filter.inputs.filter('[name=&quot;mz_fq&quot;]').val()){
$('.content-refine-search').attr('style', 'display: none !important');
} else {
$('.content-refine-search').attr('style', '');
}
}
});
// Price slider
var price_slider = $(&quot;#mz-filter-0 [data-role='rangeslider']&quot;).slider({
range: true,
min: parseFloat($('#mz-filter-0 [name=&quot;mz_fp[min]&quot;]').attr('min')),
max: parseFloat($('#mz-filter-0 [name=&quot;mz_fp[max]&quot;]').attr('max')),
values: [parseFloat($('#mz-filter-0 [name=&quot;mz_fp[min]&quot;]').val()), parseFloat($('#mz-filter-0 [name=&quot;mz_fp[max]&quot;]').val())],
slide: function( event, ui ) {
$('#mz-filter-0 [name=&quot;mz_fp[min]&quot;]').val(ui.values[0]);
$('#mz-filter-0 [name=&quot;mz_fp[max]&quot;]').val(ui.values[1]);
},
change: function( event, ui ) {
// Hide Reset for price
if($('#mz-filter-0 [name=&quot;mz_fp[min]&quot;]').val() !== $('#mz-filter-0 [name=&quot;mz_fp[min]&quot;]').attr('min') || $('#mz-filter-0 [name=&quot;mz_fp[max]&quot;]').val() !== $('#mz-filter-0 [name=&quot;mz_fp[max]&quot;]').attr('max')){
$('#mz-filter-0 [data-mz-reset=&quot;price&quot;]').removeClass('d-none');
} else {
$('#mz-filter-0 [data-mz-reset=&quot;price&quot;]').addClass('d-none');
}
// Trigger filter change
$('#mz-filter-0').change();
}
});
$('#mz-filter-0 [name=&quot;mz_fp[min]&quot;]').change(function(){
price_slider.slider(&quot;values&quot;, 0, $(this).val());
});
$('#mz-filter-0 [name=&quot;mz_fp[max]&quot;]').change(function(){
price_slider.slider(&quot;values&quot;, 1, $(this).val());
});
// Show reset all button if filter is selected
if($('#mz-filter-0 [data-mz-reset]:not(.d-none)').length){
$('[data-mz-reset=&quot;all&quot;]').removeClass('d-none');
}
}); Desktops (75) Laptops (75) Components (75) Tablets (75) Software (75) Phones &amp; PDAs (75) Cameras (75) MP3 Players (75) © LambdaTest - Powered by OpenCartmenu {
&quot;@context&quot;: &quot;https://schema.org&quot;,
&quot;@type&quot;: &quot;BreadcrumbList&quot;,
&quot;itemListElement&quot;: [
{
&quot;@type&quot;: &quot;ListItem&quot;,
&quot;position&quot;: 1,
&quot;name&quot;: &quot;Home&quot;,
&quot;item&quot;: &quot;https://ecommerce-playground.lambdatest.io/index.php?route=common/home&quot;
},
{
&quot;@type&quot;: &quot;ListItem&quot;,
&quot;position&quot;: 2,
&quot;name&quot;: &quot;Tablets&quot;
}
]
}$('#back-to-top').click(function(e){
e.preventDefault();$('html, body').animate({scrollTop: 0}, 800);
});
window.addEventListener(&quot;scroll&quot;, function(){
var el = $('#back-to-top');
if((window.pageYOffset > window.innerHeight) &amp;&amp; !el.data('show')){
el.data('show', 1); el.fadeIn();
} else if((window.pageYOffset &lt;= window.innerHeight) &amp;&amp; el.data('show')){
el.data('show', 0); el.fadeOut();
}
});id(&quot;mz-filter-panel-0-0&quot;)/div[@class=&quot;mz-filter-group-content&quot;]/div[@class=&quot;d-flex align-items-center&quot;]/input[@class=&quot;form-control&quot;]</value>
      <webElementGuid>234335e3-17be-45d8-9fce-539b7f24e351</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;product-category&quot;]</value>
      <webElementGuid>989a275c-bdbc-4588-a786-5e664bfcc941</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
      <webElementGuid>2108c3e4-281e-447c-a20b-d1fadf71926d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>dc157b5d-bcc9-4b31-ae70-08d215c58316</webElementGuid>
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
 

  Place here any module, widget, design or HTML. for example menu, categories  Cart  Your shopping cart is empty!  Sub-Total: $0.00   Total: $0.00    Edit cart  Checkout   Filter   
  Price
        to     
  Manufacturer
  

  


Apple
 
42
 


Canon
 
10
 


Hewlett-Packard
 
10
 


HTC
 
8
 


Nikon
 
2
 


Palm
 
2
 


Sony
 
1
 See more   
  Search
        
  Color
      
 

   
 

   
 

   
 

   
 

   
 

   
 

   
 

     
  Availability
      

In stock 
73
 

Pre-Order 
2
   
  Size
      
 
L
   
 
M
   
 
S
   
 
XL
   
 
XXL
     
  Discount
    
 
 10% off or more  0

 
 20% off or more  0

 
 30% off or more  0

 
 40% off or more  0

 
 50% off or more  0
   
  Rating
    
 

        &amp; up
  0

 

        &amp; up
  0

 

        &amp; up
  0

 

        &amp; up
  0

  
$(function(){
// hide refine search if sub category filter selected
if($(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fsc&quot;]:checked&quot; , &quot;'&quot; , &quot;).length || $(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fq&quot;]&quot; , &quot;'&quot; , &quot;).val()){
$(&quot; , &quot;'&quot; , &quot;.content-refine-search&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;display: none !important&quot; , &quot;'&quot; , &quot;);
}
// Filter
var paginationContainer = $(&quot; , &quot;'&quot; , &quot;.content-pagination&quot; , &quot;'&quot; , &quot;);
var productContainer = $(&quot; , &quot;'&quot; , &quot;.content-products&quot; , &quot;'&quot; , &quot;);
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1&quot; , &quot;'&quot; , &quot;).mz_filter({
requestURL: &quot;index.php?route=extension/module/mz_filter/product&amp;path=57&amp;sub_category=1&amp;_count_product&amp;_f_manufacturer&amp;_f_stock_status&amp;_f_rating&amp;_f_discount&amp;_filter_require_category&amp;_f_custom&amp;target_route=product/category&amp;module_id=95&amp;product_entry_id=&quot; + $(&quot; , &quot;'&quot; , &quot;.content-products&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot;&amp;pagination_entry_id=&quot; + $(&quot; , &quot;'&quot; , &quot;.content-pagination&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;),
searchEl: $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 .mz-filter-group-search input&quot; , &quot;'&quot; , &quot;),
ajax: true,
delay: true,
hideZeroFilter: false,
search_in_description: true,
countProduct: true,
sortBy: &quot; , &quot;'&quot; , &quot;product&quot; , &quot;'&quot; , &quot;,
onParamChange: function(param){
$(&quot;.content-limit,.content-sort-by&quot;).find(&quot; , &quot;'&quot; , &quot;option&quot; , &quot;'&quot; , &quot;).each(function(){
var url = $(this).attr(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;);
$(this).attr(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, modifyURLQuery(url, $.extend({}, param, {page: null})));
});
var currency = $(&quot; , &quot;'&quot; , &quot;#form-currency input[name=&quot;redirect&quot;]&quot; , &quot;'&quot; , &quot;);
currency.val(modifyURLQuery(currency.val(), $.extend({}, param, {mz_fp: null, page: null})));
// Show or hide reset all button
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 .mz-filter-group [data-mz-reset]:not(.d-none)&quot; , &quot;'&quot; , &quot;).length){
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [data-mz-reset=&quot;all&quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [data-mz-reset=&quot;all&quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
},
onInputChange: function(e){
var filter_group = $(e.target).closest(&quot; , &quot;'&quot; , &quot;.mz-filter-group&quot; , &quot;'&quot; , &quot;);
var is_input_selected = false;
// Hide Reset for Checkbox or radio
if(filter_group.find(&quot; , &quot;'&quot; , &quot;input[type=&quot;checkbox&quot;]:checked,input[type=&quot;radio&quot;]:checked&quot; , &quot;'&quot; , &quot;).length){
is_input_selected = true;
}
// Hide Reset for price
if($(e.target).filter(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fp[min]&quot;],[name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).length){
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;) || $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;)){
is_input_selected = true;
}
}
// Hide reset for text
if($(e.target).filter(&quot; , &quot;'&quot; , &quot;[type=&quot;text&quot;]&quot; , &quot;'&quot; , &quot;).val()){
is_input_selected = true;
}
// Hide or show reset buton
if(is_input_selected){
filter_group.find(&quot; , &quot;'&quot; , &quot;[data-mz-reset]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
} else {
filter_group.find(&quot; , &quot;'&quot; , &quot;[data-mz-reset]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
},
onReset: function(type){
// Reset price
if(type === &quot; , &quot;'&quot; , &quot;price&quot; , &quot;'&quot; , &quot; || type === &quot; , &quot;'&quot; , &quot;all&quot; , &quot;'&quot; , &quot;){
price_slider.slider(&quot;values&quot;, [parseFloat(price_slider.slider(&quot;option&quot;, &quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;)), parseFloat(price_slider.slider(&quot;option&quot;, &quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;))]);
}
},
onBeforeSend: function(){
productContainer.append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;mz-filter-loader&quot;>&lt;div class=&quot;loader-spinner&quot;>&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);
},
onResult: function(json, mz_filter){
// Add result products to container
if(json[&quot; , &quot;'&quot; , &quot;products&quot; , &quot;'&quot; , &quot;]){
productContainer.find(&quot; , &quot;'&quot; , &quot;[data-countdown]&quot; , &quot;'&quot; , &quot;).countdown(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
productContainer.html(json[&quot; , &quot;'&quot; , &quot;products&quot; , &quot;'&quot; , &quot;]);
pageLoad(productContainer);
productContainer.find(&quot; , &quot;'&quot; , &quot;#button-continue&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
e.preventDefault();
mz_filter.resetAll();
});
$(&quot; , &quot;'&quot; , &quot;#list-view.active&quot; , &quot;'&quot; , &quot;).click();
$(&quot; , &quot;'&quot; , &quot;#grid-view.active&quot; , &quot;'&quot; , &quot;).click();
} else {
productContainer.html(&quot;&lt;div class=&quot; , &quot;'&quot; , &quot;col-xs-12 text-center&quot; , &quot;'&quot; , &quot;>No products found!&lt;/div>&quot;);
}
// Add pagination to container
if(json[&quot; , &quot;'&quot; , &quot;pagination&quot; , &quot;'&quot; , &quot;]){
paginationContainer.html(json[&quot; , &quot;'&quot; , &quot;pagination&quot; , &quot;'&quot; , &quot;]);
} else {
paginationContainer.empty();
}
// hide refine search if sub category filter selected
if(mz_filter.inputs.filter(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fsc&quot;]:checked&quot; , &quot;'&quot; , &quot;).length || mz_filter.inputs.filter(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fq&quot;]&quot; , &quot;'&quot; , &quot;).val()){
$(&quot; , &quot;'&quot; , &quot;.content-refine-search&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;display: none !important&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;.content-refine-search&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
}
}
});
// Price slider
var price_slider = $(&quot;#mz-filter-1 [data-role=&quot; , &quot;'&quot; , &quot;rangeslider&quot; , &quot;'&quot; , &quot;]&quot;).slider({
range: true,
min: parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;)),
max: parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;)),
values: [parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val()), parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val())],
slide: function( event, ui ) {
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.values[0]);
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.values[1]);
},
change: function( event, ui ) {
// Hide Reset for price
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;) || $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;)){
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [data-mz-reset=&quot;price&quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [data-mz-reset=&quot;price&quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
// Trigger filter change
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1&quot; , &quot;'&quot; , &quot;).change();
}
});
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).change(function(){
price_slider.slider(&quot;values&quot;, 0, $(this).val());
});
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).change(function(){
price_slider.slider(&quot;values&quot;, 1, $(this).val());
});
// Show reset all button if filter is selected
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [data-mz-reset]:not(.d-none)&quot; , &quot;'&quot; , &quot;).length){
$(&quot; , &quot;'&quot; , &quot;[data-mz-reset=&quot;all&quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
});                   All Categories  All Categories Desktops Laptops Components Tablets Software Phones &amp; PDAs Cameras MP3 Players
          Search          0   0 item(s) - $0.00          0   0 item(s) - $0.00    Shop by Category     
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
             This is a dummy website for Web Automation Testing Upto 60% Off on Smartbuy Mobile Accessories, Small Appliances, Automotive Accessories &amp; more SAVE60      Tablets Tablets
 Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Semper auctor neque vitae tempus quam pellentesque. Facilisis leo vel fringilla est ullamcorper. Tellus id interdum velit laoreet id donec ultrices tincidunt. Vitae nunc sed velit dignissim sodales. Mi proin sed libero enim sed. Mi quis hendrerit dolor magna eget est lorem ipsum.  Product Compare (0)  Filter Show:  15 25 50 75 100   Sort By:  Default Best sellers Popular Newest Name (A - Z) Name (Z - A) Price (Low > High) Price (High > Low) Rating (Highest) Rating (Lowest) Model (A - Z) Model (Z - A)   Show:  15 25 50 75 100   Sort By:  Default Best sellers Popular Newest Name (A - Z) Name (Z - A) Price (Low > High) Price (High > Low) Rating (Highest) Rating (Lowest) Model (A - Z) Model (Z - A)  There are no products to list in this category.

 Continue

  Filter    
  Price
        to     
  Manufacturer
  

    



Apple
 
0
 



Canon
 
0
 



Hewlett-Packard
 
0
 



HTC
 
0
 



Nikon
 
0
 



Palm
 
0
 



Sony
 
0
See more  
  Search
        
  Color
       
 

   
 

   
 

   
 

   
 

   
 

   
 

   
 

    
  Availability
       

In stock 
0
 

Pre-Order 
0
  
  Size
       
 
L
   
 
M
   
 
S
   
 
XL
   
 
XXL
    
  Discount
    
 
 10% off or more  0

 
 20% off or more  0

 
 30% off or more  0

 
 40% off or more  0

 
 50% off or more  0
   
  Rating
    
 

        &amp; up
  0

 

        &amp; up
  0

 

        &amp; up
  0

 

        &amp; up
  0

  
$(function(){
if(window.innerWidth &lt; 767){ // Collaped all panel in small device
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 .collapse.show&quot; , &quot;'&quot; , &quot;).collapse(&quot;hide&quot;);
}
// hide refine search if sub category filter selected
if($(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fsc&quot;]:checked&quot; , &quot;'&quot; , &quot;).length || $(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fq&quot;]&quot; , &quot;'&quot; , &quot;).val()){
$(&quot; , &quot;'&quot; , &quot;.content-refine-search&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;display: none !important&quot; , &quot;'&quot; , &quot;);
}
// Filter
var paginationContainer = $(&quot; , &quot;'&quot; , &quot;.content-pagination&quot; , &quot;'&quot; , &quot;);
var productContainer = $(&quot; , &quot;'&quot; , &quot;.content-products&quot; , &quot;'&quot; , &quot;);
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0&quot; , &quot;'&quot; , &quot;).mz_filter({
requestURL: &quot;index.php?route=extension/module/mz_filter/product&amp;path=57&amp;sub_category=1&amp;_count_product&amp;_f_manufacturer&amp;_f_sub_category&amp;_f_stock_status&amp;_f_rating&amp;_f_discount&amp;_filter_require_category&amp;_f_custom&amp;target_route=product/category&amp;module_id=80&amp;product_entry_id=&quot; + $(&quot; , &quot;'&quot; , &quot;.content-products&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot;&amp;pagination_entry_id=&quot; + $(&quot; , &quot;'&quot; , &quot;.content-pagination&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;),
searchEl: $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 .mz-filter-group-search input&quot; , &quot;'&quot; , &quot;),
ajax: true,
delay: true,
hideZeroFilter: false,
search_in_description: true,
countProduct: true,
sortBy: &quot; , &quot;'&quot; , &quot;product&quot; , &quot;'&quot; , &quot;,
onParamChange: function(param){
$(&quot;.content-limit,.content-sort-by&quot;).find(&quot; , &quot;'&quot; , &quot;option&quot; , &quot;'&quot; , &quot;).each(function(){
var url = $(this).attr(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;);
$(this).attr(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, modifyURLQuery(url, $.extend({}, param, {page: null})));
});
var currency = $(&quot; , &quot;'&quot; , &quot;#form-currency input[name=&quot;redirect&quot;]&quot; , &quot;'&quot; , &quot;);
currency.val(modifyURLQuery(currency.val(), $.extend({}, param, {mz_fp: null, page: null})));
// Show or hide reset all button
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 .mz-filter-group [data-mz-reset]:not(.d-none)&quot; , &quot;'&quot; , &quot;).length){
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [data-mz-reset=&quot;all&quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [data-mz-reset=&quot;all&quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
},
onInputChange: function(e){
var filter_group = $(e.target).closest(&quot; , &quot;'&quot; , &quot;.mz-filter-group&quot; , &quot;'&quot; , &quot;);
var is_input_selected = false;
// Hide Reset for Checkbox or radio
if(filter_group.find(&quot; , &quot;'&quot; , &quot;input[type=&quot;checkbox&quot;]:checked,input[type=&quot;radio&quot;]:checked&quot; , &quot;'&quot; , &quot;).length){
is_input_selected = true;
}
// Hide Reset for price
if($(e.target).filter(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fp[min]&quot;],[name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).length){
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;) || $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;)){
is_input_selected = true;
}
}
// Hide reset for text
if($(e.target).filter(&quot; , &quot;'&quot; , &quot;[type=&quot;text&quot;]&quot; , &quot;'&quot; , &quot;).val()){
is_input_selected = true;
}
// Hide or show reset buton
if(is_input_selected){
filter_group.find(&quot; , &quot;'&quot; , &quot;[data-mz-reset]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
} else {
filter_group.find(&quot; , &quot;'&quot; , &quot;[data-mz-reset]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
},
onReset: function(type){
// Reset price
if(type === &quot; , &quot;'&quot; , &quot;price&quot; , &quot;'&quot; , &quot; || type === &quot; , &quot;'&quot; , &quot;all&quot; , &quot;'&quot; , &quot;){
price_slider.slider(&quot;values&quot;, [parseFloat(price_slider.slider(&quot;option&quot;, &quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;)), parseFloat(price_slider.slider(&quot;option&quot;, &quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;))]);
}
},
onBeforeSend: function(){
productContainer.append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;mz-filter-loader&quot;>&lt;div class=&quot;loader-spinner&quot;>&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);
},
onResult: function(json, mz_filter){
// Add result products to container
if(json[&quot; , &quot;'&quot; , &quot;products&quot; , &quot;'&quot; , &quot;]){
productContainer.find(&quot; , &quot;'&quot; , &quot;[data-countdown]&quot; , &quot;'&quot; , &quot;).countdown(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
productContainer.html(json[&quot; , &quot;'&quot; , &quot;products&quot; , &quot;'&quot; , &quot;]);
pageLoad(productContainer);
productContainer.find(&quot; , &quot;'&quot; , &quot;#button-continue&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
e.preventDefault();
mz_filter.resetAll();
});
$(&quot; , &quot;'&quot; , &quot;#list-view.active&quot; , &quot;'&quot; , &quot;).click();
$(&quot; , &quot;'&quot; , &quot;#grid-view.active&quot; , &quot;'&quot; , &quot;).click();
} else {
productContainer.html(&quot;&lt;div class=&quot; , &quot;'&quot; , &quot;col-xs-12 text-center&quot; , &quot;'&quot; , &quot;>No products found!&lt;/div>&quot;);
}
// Add pagination to container
if(json[&quot; , &quot;'&quot; , &quot;pagination&quot; , &quot;'&quot; , &quot;]){
paginationContainer.html(json[&quot; , &quot;'&quot; , &quot;pagination&quot; , &quot;'&quot; , &quot;]);
} else {
paginationContainer.empty();
}
// hide refine search if sub category filter selected
if(mz_filter.inputs.filter(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fsc&quot;]:checked&quot; , &quot;'&quot; , &quot;).length || mz_filter.inputs.filter(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fq&quot;]&quot; , &quot;'&quot; , &quot;).val()){
$(&quot; , &quot;'&quot; , &quot;.content-refine-search&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;display: none !important&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;.content-refine-search&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
}
}
});
// Price slider
var price_slider = $(&quot;#mz-filter-0 [data-role=&quot; , &quot;'&quot; , &quot;rangeslider&quot; , &quot;'&quot; , &quot;]&quot;).slider({
range: true,
min: parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;)),
max: parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;)),
values: [parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val()), parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val())],
slide: function( event, ui ) {
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.values[0]);
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.values[1]);
},
change: function( event, ui ) {
// Hide Reset for price
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;) || $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;)){
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [data-mz-reset=&quot;price&quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [data-mz-reset=&quot;price&quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
// Trigger filter change
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0&quot; , &quot;'&quot; , &quot;).change();
}
});
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).change(function(){
price_slider.slider(&quot;values&quot;, 0, $(this).val());
});
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).change(function(){
price_slider.slider(&quot;values&quot;, 1, $(this).val());
});
// Show reset all button if filter is selected
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [data-mz-reset]:not(.d-none)&quot; , &quot;'&quot; , &quot;).length){
$(&quot; , &quot;'&quot; , &quot;[data-mz-reset=&quot;all&quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
}); Desktops (75) Laptops (75) Components (75) Tablets (75) Software (75) Phones &amp; PDAs (75) Cameras (75) MP3 Players (75) © LambdaTest - Powered by OpenCartmenu {
&quot;@context&quot;: &quot;https://schema.org&quot;,
&quot;@type&quot;: &quot;BreadcrumbList&quot;,
&quot;itemListElement&quot;: [
{
&quot;@type&quot;: &quot;ListItem&quot;,
&quot;position&quot;: 1,
&quot;name&quot;: &quot;Home&quot;,
&quot;item&quot;: &quot;https://ecommerce-playground.lambdatest.io/index.php?route=common/home&quot;
},
{
&quot;@type&quot;: &quot;ListItem&quot;,
&quot;position&quot;: 2,
&quot;name&quot;: &quot;Tablets&quot;
}
]
}$(&quot; , &quot;'&quot; , &quot;#back-to-top&quot; , &quot;'&quot; , &quot;).click(function(e){
e.preventDefault();$(&quot; , &quot;'&quot; , &quot;html, body&quot; , &quot;'&quot; , &quot;).animate({scrollTop: 0}, 800);
});
window.addEventListener(&quot;scroll&quot;, function(){
var el = $(&quot; , &quot;'&quot; , &quot;#back-to-top&quot; , &quot;'&quot; , &quot;);
if((window.pageYOffset > window.innerHeight) &amp;&amp; !el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;)){
el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;, 1); el.fadeIn();
} else if((window.pageYOffset &lt;= window.innerHeight) &amp;&amp; el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;)){
el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;, 0); el.fadeOut();
}
});id(&quot;mz-filter-panel-0-0&quot;)/div[@class=&quot;mz-filter-group-content&quot;]/div[@class=&quot;d-flex align-items-center&quot;]/input[@class=&quot;form-control&quot;]&quot;) or . = concat(&quot;   Top categories      
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
 

  Place here any module, widget, design or HTML. for example menu, categories  Cart  Your shopping cart is empty!  Sub-Total: $0.00   Total: $0.00    Edit cart  Checkout   Filter   
  Price
        to     
  Manufacturer
  

  


Apple
 
42
 


Canon
 
10
 


Hewlett-Packard
 
10
 


HTC
 
8
 


Nikon
 
2
 


Palm
 
2
 


Sony
 
1
 See more   
  Search
        
  Color
      
 

   
 

   
 

   
 

   
 

   
 

   
 

   
 

     
  Availability
      

In stock 
73
 

Pre-Order 
2
   
  Size
      
 
L
   
 
M
   
 
S
   
 
XL
   
 
XXL
     
  Discount
    
 
 10% off or more  0

 
 20% off or more  0

 
 30% off or more  0

 
 40% off or more  0

 
 50% off or more  0
   
  Rating
    
 

        &amp; up
  0

 

        &amp; up
  0

 

        &amp; up
  0

 

        &amp; up
  0

  
$(function(){
// hide refine search if sub category filter selected
if($(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fsc&quot;]:checked&quot; , &quot;'&quot; , &quot;).length || $(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fq&quot;]&quot; , &quot;'&quot; , &quot;).val()){
$(&quot; , &quot;'&quot; , &quot;.content-refine-search&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;display: none !important&quot; , &quot;'&quot; , &quot;);
}
// Filter
var paginationContainer = $(&quot; , &quot;'&quot; , &quot;.content-pagination&quot; , &quot;'&quot; , &quot;);
var productContainer = $(&quot; , &quot;'&quot; , &quot;.content-products&quot; , &quot;'&quot; , &quot;);
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1&quot; , &quot;'&quot; , &quot;).mz_filter({
requestURL: &quot;index.php?route=extension/module/mz_filter/product&amp;path=57&amp;sub_category=1&amp;_count_product&amp;_f_manufacturer&amp;_f_stock_status&amp;_f_rating&amp;_f_discount&amp;_filter_require_category&amp;_f_custom&amp;target_route=product/category&amp;module_id=95&amp;product_entry_id=&quot; + $(&quot; , &quot;'&quot; , &quot;.content-products&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot;&amp;pagination_entry_id=&quot; + $(&quot; , &quot;'&quot; , &quot;.content-pagination&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;),
searchEl: $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 .mz-filter-group-search input&quot; , &quot;'&quot; , &quot;),
ajax: true,
delay: true,
hideZeroFilter: false,
search_in_description: true,
countProduct: true,
sortBy: &quot; , &quot;'&quot; , &quot;product&quot; , &quot;'&quot; , &quot;,
onParamChange: function(param){
$(&quot;.content-limit,.content-sort-by&quot;).find(&quot; , &quot;'&quot; , &quot;option&quot; , &quot;'&quot; , &quot;).each(function(){
var url = $(this).attr(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;);
$(this).attr(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, modifyURLQuery(url, $.extend({}, param, {page: null})));
});
var currency = $(&quot; , &quot;'&quot; , &quot;#form-currency input[name=&quot;redirect&quot;]&quot; , &quot;'&quot; , &quot;);
currency.val(modifyURLQuery(currency.val(), $.extend({}, param, {mz_fp: null, page: null})));
// Show or hide reset all button
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 .mz-filter-group [data-mz-reset]:not(.d-none)&quot; , &quot;'&quot; , &quot;).length){
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [data-mz-reset=&quot;all&quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [data-mz-reset=&quot;all&quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
},
onInputChange: function(e){
var filter_group = $(e.target).closest(&quot; , &quot;'&quot; , &quot;.mz-filter-group&quot; , &quot;'&quot; , &quot;);
var is_input_selected = false;
// Hide Reset for Checkbox or radio
if(filter_group.find(&quot; , &quot;'&quot; , &quot;input[type=&quot;checkbox&quot;]:checked,input[type=&quot;radio&quot;]:checked&quot; , &quot;'&quot; , &quot;).length){
is_input_selected = true;
}
// Hide Reset for price
if($(e.target).filter(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fp[min]&quot;],[name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).length){
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;) || $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;)){
is_input_selected = true;
}
}
// Hide reset for text
if($(e.target).filter(&quot; , &quot;'&quot; , &quot;[type=&quot;text&quot;]&quot; , &quot;'&quot; , &quot;).val()){
is_input_selected = true;
}
// Hide or show reset buton
if(is_input_selected){
filter_group.find(&quot; , &quot;'&quot; , &quot;[data-mz-reset]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
} else {
filter_group.find(&quot; , &quot;'&quot; , &quot;[data-mz-reset]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
},
onReset: function(type){
// Reset price
if(type === &quot; , &quot;'&quot; , &quot;price&quot; , &quot;'&quot; , &quot; || type === &quot; , &quot;'&quot; , &quot;all&quot; , &quot;'&quot; , &quot;){
price_slider.slider(&quot;values&quot;, [parseFloat(price_slider.slider(&quot;option&quot;, &quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;)), parseFloat(price_slider.slider(&quot;option&quot;, &quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;))]);
}
},
onBeforeSend: function(){
productContainer.append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;mz-filter-loader&quot;>&lt;div class=&quot;loader-spinner&quot;>&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);
},
onResult: function(json, mz_filter){
// Add result products to container
if(json[&quot; , &quot;'&quot; , &quot;products&quot; , &quot;'&quot; , &quot;]){
productContainer.find(&quot; , &quot;'&quot; , &quot;[data-countdown]&quot; , &quot;'&quot; , &quot;).countdown(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
productContainer.html(json[&quot; , &quot;'&quot; , &quot;products&quot; , &quot;'&quot; , &quot;]);
pageLoad(productContainer);
productContainer.find(&quot; , &quot;'&quot; , &quot;#button-continue&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
e.preventDefault();
mz_filter.resetAll();
});
$(&quot; , &quot;'&quot; , &quot;#list-view.active&quot; , &quot;'&quot; , &quot;).click();
$(&quot; , &quot;'&quot; , &quot;#grid-view.active&quot; , &quot;'&quot; , &quot;).click();
} else {
productContainer.html(&quot;&lt;div class=&quot; , &quot;'&quot; , &quot;col-xs-12 text-center&quot; , &quot;'&quot; , &quot;>No products found!&lt;/div>&quot;);
}
// Add pagination to container
if(json[&quot; , &quot;'&quot; , &quot;pagination&quot; , &quot;'&quot; , &quot;]){
paginationContainer.html(json[&quot; , &quot;'&quot; , &quot;pagination&quot; , &quot;'&quot; , &quot;]);
} else {
paginationContainer.empty();
}
// hide refine search if sub category filter selected
if(mz_filter.inputs.filter(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fsc&quot;]:checked&quot; , &quot;'&quot; , &quot;).length || mz_filter.inputs.filter(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fq&quot;]&quot; , &quot;'&quot; , &quot;).val()){
$(&quot; , &quot;'&quot; , &quot;.content-refine-search&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;display: none !important&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;.content-refine-search&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
}
}
});
// Price slider
var price_slider = $(&quot;#mz-filter-1 [data-role=&quot; , &quot;'&quot; , &quot;rangeslider&quot; , &quot;'&quot; , &quot;]&quot;).slider({
range: true,
min: parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;)),
max: parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;)),
values: [parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val()), parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val())],
slide: function( event, ui ) {
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.values[0]);
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.values[1]);
},
change: function( event, ui ) {
// Hide Reset for price
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;) || $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;)){
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [data-mz-reset=&quot;price&quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [data-mz-reset=&quot;price&quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
// Trigger filter change
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1&quot; , &quot;'&quot; , &quot;).change();
}
});
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).change(function(){
price_slider.slider(&quot;values&quot;, 0, $(this).val());
});
$(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).change(function(){
price_slider.slider(&quot;values&quot;, 1, $(this).val());
});
// Show reset all button if filter is selected
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-1 [data-mz-reset]:not(.d-none)&quot; , &quot;'&quot; , &quot;).length){
$(&quot; , &quot;'&quot; , &quot;[data-mz-reset=&quot;all&quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
});                   All Categories  All Categories Desktops Laptops Components Tablets Software Phones &amp; PDAs Cameras MP3 Players
          Search          0   0 item(s) - $0.00          0   0 item(s) - $0.00    Shop by Category     
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
             This is a dummy website for Web Automation Testing Upto 60% Off on Smartbuy Mobile Accessories, Small Appliances, Automotive Accessories &amp; more SAVE60      Tablets Tablets
 Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Semper auctor neque vitae tempus quam pellentesque. Facilisis leo vel fringilla est ullamcorper. Tellus id interdum velit laoreet id donec ultrices tincidunt. Vitae nunc sed velit dignissim sodales. Mi proin sed libero enim sed. Mi quis hendrerit dolor magna eget est lorem ipsum.  Product Compare (0)  Filter Show:  15 25 50 75 100   Sort By:  Default Best sellers Popular Newest Name (A - Z) Name (Z - A) Price (Low > High) Price (High > Low) Rating (Highest) Rating (Lowest) Model (A - Z) Model (Z - A)   Show:  15 25 50 75 100   Sort By:  Default Best sellers Popular Newest Name (A - Z) Name (Z - A) Price (Low > High) Price (High > Low) Rating (Highest) Rating (Lowest) Model (A - Z) Model (Z - A)  There are no products to list in this category.

 Continue

  Filter    
  Price
        to     
  Manufacturer
  

    



Apple
 
0
 



Canon
 
0
 



Hewlett-Packard
 
0
 



HTC
 
0
 



Nikon
 
0
 



Palm
 
0
 



Sony
 
0
See more  
  Search
        
  Color
       
 

   
 

   
 

   
 

   
 

   
 

   
 

   
 

    
  Availability
       

In stock 
0
 

Pre-Order 
0
  
  Size
       
 
L
   
 
M
   
 
S
   
 
XL
   
 
XXL
    
  Discount
    
 
 10% off or more  0

 
 20% off or more  0

 
 30% off or more  0

 
 40% off or more  0

 
 50% off or more  0
   
  Rating
    
 

        &amp; up
  0

 

        &amp; up
  0

 

        &amp; up
  0

 

        &amp; up
  0

  
$(function(){
if(window.innerWidth &lt; 767){ // Collaped all panel in small device
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 .collapse.show&quot; , &quot;'&quot; , &quot;).collapse(&quot;hide&quot;);
}
// hide refine search if sub category filter selected
if($(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fsc&quot;]:checked&quot; , &quot;'&quot; , &quot;).length || $(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fq&quot;]&quot; , &quot;'&quot; , &quot;).val()){
$(&quot; , &quot;'&quot; , &quot;.content-refine-search&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;display: none !important&quot; , &quot;'&quot; , &quot;);
}
// Filter
var paginationContainer = $(&quot; , &quot;'&quot; , &quot;.content-pagination&quot; , &quot;'&quot; , &quot;);
var productContainer = $(&quot; , &quot;'&quot; , &quot;.content-products&quot; , &quot;'&quot; , &quot;);
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0&quot; , &quot;'&quot; , &quot;).mz_filter({
requestURL: &quot;index.php?route=extension/module/mz_filter/product&amp;path=57&amp;sub_category=1&amp;_count_product&amp;_f_manufacturer&amp;_f_sub_category&amp;_f_stock_status&amp;_f_rating&amp;_f_discount&amp;_filter_require_category&amp;_f_custom&amp;target_route=product/category&amp;module_id=80&amp;product_entry_id=&quot; + $(&quot; , &quot;'&quot; , &quot;.content-products&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot;&amp;pagination_entry_id=&quot; + $(&quot; , &quot;'&quot; , &quot;.content-pagination&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;),
searchEl: $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 .mz-filter-group-search input&quot; , &quot;'&quot; , &quot;),
ajax: true,
delay: true,
hideZeroFilter: false,
search_in_description: true,
countProduct: true,
sortBy: &quot; , &quot;'&quot; , &quot;product&quot; , &quot;'&quot; , &quot;,
onParamChange: function(param){
$(&quot;.content-limit,.content-sort-by&quot;).find(&quot; , &quot;'&quot; , &quot;option&quot; , &quot;'&quot; , &quot;).each(function(){
var url = $(this).attr(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;);
$(this).attr(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, modifyURLQuery(url, $.extend({}, param, {page: null})));
});
var currency = $(&quot; , &quot;'&quot; , &quot;#form-currency input[name=&quot;redirect&quot;]&quot; , &quot;'&quot; , &quot;);
currency.val(modifyURLQuery(currency.val(), $.extend({}, param, {mz_fp: null, page: null})));
// Show or hide reset all button
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 .mz-filter-group [data-mz-reset]:not(.d-none)&quot; , &quot;'&quot; , &quot;).length){
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [data-mz-reset=&quot;all&quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [data-mz-reset=&quot;all&quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
},
onInputChange: function(e){
var filter_group = $(e.target).closest(&quot; , &quot;'&quot; , &quot;.mz-filter-group&quot; , &quot;'&quot; , &quot;);
var is_input_selected = false;
// Hide Reset for Checkbox or radio
if(filter_group.find(&quot; , &quot;'&quot; , &quot;input[type=&quot;checkbox&quot;]:checked,input[type=&quot;radio&quot;]:checked&quot; , &quot;'&quot; , &quot;).length){
is_input_selected = true;
}
// Hide Reset for price
if($(e.target).filter(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fp[min]&quot;],[name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).length){
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;) || $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;)){
is_input_selected = true;
}
}
// Hide reset for text
if($(e.target).filter(&quot; , &quot;'&quot; , &quot;[type=&quot;text&quot;]&quot; , &quot;'&quot; , &quot;).val()){
is_input_selected = true;
}
// Hide or show reset buton
if(is_input_selected){
filter_group.find(&quot; , &quot;'&quot; , &quot;[data-mz-reset]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
} else {
filter_group.find(&quot; , &quot;'&quot; , &quot;[data-mz-reset]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
},
onReset: function(type){
// Reset price
if(type === &quot; , &quot;'&quot; , &quot;price&quot; , &quot;'&quot; , &quot; || type === &quot; , &quot;'&quot; , &quot;all&quot; , &quot;'&quot; , &quot;){
price_slider.slider(&quot;values&quot;, [parseFloat(price_slider.slider(&quot;option&quot;, &quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;)), parseFloat(price_slider.slider(&quot;option&quot;, &quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;))]);
}
},
onBeforeSend: function(){
productContainer.append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;mz-filter-loader&quot;>&lt;div class=&quot;loader-spinner&quot;>&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);
},
onResult: function(json, mz_filter){
// Add result products to container
if(json[&quot; , &quot;'&quot; , &quot;products&quot; , &quot;'&quot; , &quot;]){
productContainer.find(&quot; , &quot;'&quot; , &quot;[data-countdown]&quot; , &quot;'&quot; , &quot;).countdown(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
productContainer.html(json[&quot; , &quot;'&quot; , &quot;products&quot; , &quot;'&quot; , &quot;]);
pageLoad(productContainer);
productContainer.find(&quot; , &quot;'&quot; , &quot;#button-continue&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
e.preventDefault();
mz_filter.resetAll();
});
$(&quot; , &quot;'&quot; , &quot;#list-view.active&quot; , &quot;'&quot; , &quot;).click();
$(&quot; , &quot;'&quot; , &quot;#grid-view.active&quot; , &quot;'&quot; , &quot;).click();
} else {
productContainer.html(&quot;&lt;div class=&quot; , &quot;'&quot; , &quot;col-xs-12 text-center&quot; , &quot;'&quot; , &quot;>No products found!&lt;/div>&quot;);
}
// Add pagination to container
if(json[&quot; , &quot;'&quot; , &quot;pagination&quot; , &quot;'&quot; , &quot;]){
paginationContainer.html(json[&quot; , &quot;'&quot; , &quot;pagination&quot; , &quot;'&quot; , &quot;]);
} else {
paginationContainer.empty();
}
// hide refine search if sub category filter selected
if(mz_filter.inputs.filter(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fsc&quot;]:checked&quot; , &quot;'&quot; , &quot;).length || mz_filter.inputs.filter(&quot; , &quot;'&quot; , &quot;[name=&quot;mz_fq&quot;]&quot; , &quot;'&quot; , &quot;).val()){
$(&quot; , &quot;'&quot; , &quot;.content-refine-search&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;display: none !important&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;.content-refine-search&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
}
}
});
// Price slider
var price_slider = $(&quot;#mz-filter-0 [data-role=&quot; , &quot;'&quot; , &quot;rangeslider&quot; , &quot;'&quot; , &quot;]&quot;).slider({
range: true,
min: parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;)),
max: parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;)),
values: [parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val()), parseFloat($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val())],
slide: function( event, ui ) {
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.values[0]);
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.values[1]);
},
change: function( event, ui ) {
// Hide Reset for price
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;min&quot; , &quot;'&quot; , &quot;) || $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).val() !== $(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;max&quot; , &quot;'&quot; , &quot;)){
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [data-mz-reset=&quot;price&quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [data-mz-reset=&quot;price&quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
// Trigger filter change
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0&quot; , &quot;'&quot; , &quot;).change();
}
});
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[min]&quot;]&quot; , &quot;'&quot; , &quot;).change(function(){
price_slider.slider(&quot;values&quot;, 0, $(this).val());
});
$(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [name=&quot;mz_fp[max]&quot;]&quot; , &quot;'&quot; , &quot;).change(function(){
price_slider.slider(&quot;values&quot;, 1, $(this).val());
});
// Show reset all button if filter is selected
if($(&quot; , &quot;'&quot; , &quot;#mz-filter-0 [data-mz-reset]:not(.d-none)&quot; , &quot;'&quot; , &quot;).length){
$(&quot; , &quot;'&quot; , &quot;[data-mz-reset=&quot;all&quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
}
}); Desktops (75) Laptops (75) Components (75) Tablets (75) Software (75) Phones &amp; PDAs (75) Cameras (75) MP3 Players (75) © LambdaTest - Powered by OpenCartmenu {
&quot;@context&quot;: &quot;https://schema.org&quot;,
&quot;@type&quot;: &quot;BreadcrumbList&quot;,
&quot;itemListElement&quot;: [
{
&quot;@type&quot;: &quot;ListItem&quot;,
&quot;position&quot;: 1,
&quot;name&quot;: &quot;Home&quot;,
&quot;item&quot;: &quot;https://ecommerce-playground.lambdatest.io/index.php?route=common/home&quot;
},
{
&quot;@type&quot;: &quot;ListItem&quot;,
&quot;position&quot;: 2,
&quot;name&quot;: &quot;Tablets&quot;
}
]
}$(&quot; , &quot;'&quot; , &quot;#back-to-top&quot; , &quot;'&quot; , &quot;).click(function(e){
e.preventDefault();$(&quot; , &quot;'&quot; , &quot;html, body&quot; , &quot;'&quot; , &quot;).animate({scrollTop: 0}, 800);
});
window.addEventListener(&quot;scroll&quot;, function(){
var el = $(&quot; , &quot;'&quot; , &quot;#back-to-top&quot; , &quot;'&quot; , &quot;);
if((window.pageYOffset > window.innerHeight) &amp;&amp; !el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;)){
el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;, 1); el.fadeIn();
} else if((window.pageYOffset &lt;= window.innerHeight) &amp;&amp; el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;)){
el.data(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;, 0); el.fadeOut();
}
});id(&quot;mz-filter-panel-0-0&quot;)/div[@class=&quot;mz-filter-group-content&quot;]/div[@class=&quot;d-flex align-items-center&quot;]/input[@class=&quot;form-control&quot;]&quot;))]</value>
      <webElementGuid>cf268136-ed67-424a-861e-bc6642248330</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
